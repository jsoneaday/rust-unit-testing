use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use crate::common::entities::base::{EntityId, DbRepo};
use super::model::{CircleGroupWithProfileQueryResult, CircleGroupMemberWithProfileQueryResult};

mod private_members {    
    use crate::common::entities::circle_group::model::{CircleGroupWithProfileQueryResult, CircleGroupMemberWithProfileQueryResult};
    use super::*;

    pub async fn insert_circle_inner(conn: &Pool<Postgres>, circle_owner_id: i64) -> Result<i64, sqlx::Error> {
        let insert_result = sqlx::query_as::<_, EntityId>("insert into circle_group (owner_id) values ($1) returning id")
            .bind(circle_owner_id)
            .fetch_one(conn)
            .await;

        match insert_result {
            Ok(row) => Ok(row.id),
            Err(e) => Err(e)
        }
    }

    pub async fn insert_circle_member_inner(conn: &Pool<Postgres>, circle_group_id: i64, new_member_id: i64) -> Result<i64, sqlx::Error> {
        let insert_result = sqlx::query_as::<_, EntityId>("insert into circle_group_member (circle_group_id, member_id) values ($1, $2) returning id")
            .bind(circle_group_id)
            .bind(new_member_id)
            .fetch_one(conn)
            .await;

        match insert_result {
            Ok(row) => Ok(row.id),
            Err(e) => Err(e)
        }
    }

    pub async fn query_circle_inner(conn: &Pool<Postgres>, id: i64) -> Result<Option<CircleGroupWithProfileQueryResult>, sqlx::Error> {
        sqlx::query_as::<_, CircleGroupWithProfileQueryResult>(
            r"
                select c.id, c.updated_at, c.owner_id, p.user_name, p.full_name, p.avatar
                from circle_group c
                    join profile p on c.owner_id = p.id
                where c.id = $1
            ")
            .bind(id)
            .fetch_optional(conn)
            .await
    }

    pub async fn query_circle_member_inner(conn: &Pool<Postgres>, id: i64) -> Result<Option<CircleGroupMemberWithProfileQueryResult>, sqlx::Error> {
        sqlx::query_as::<_, CircleGroupMemberWithProfileQueryResult>(
            r"
                select c.id, c.updated_at, c.circle_group_id, p.id as member_id, p.user_name, p.full_name, p.avatar
                from circle_group_member c
                    join profile p on c.member_id = p.id
                where c.id = $1
            ")
            .bind(id)
            .fetch_optional(conn)
            .await
    }
}

#[async_trait]
pub trait InsertCircleFn {
    async fn insert_circle(&self, conn: &Pool<Postgres>, circle_owner_id: i64) -> Result<i64, sqlx::Error>;
}

#[async_trait]
impl InsertCircleFn for DbRepo {
    async fn insert_circle(&self, conn: &Pool<Postgres>, circle_owner_id: i64) -> Result<i64, sqlx::Error> {
        private_members::insert_circle_inner(conn, circle_owner_id).await
    }
}

#[async_trait]
pub trait InsertCircleMemberFn {
    async fn insert_circle_member(&self, conn: &Pool<Postgres>, circle_group_id: i64, new_member_id: i64) -> Result<i64, sqlx::Error>;
}

#[async_trait]
impl InsertCircleMemberFn for DbRepo {
    async fn insert_circle_member(&self, conn: &Pool<Postgres>, circle_group_id: i64, new_member_id: i64) -> Result<i64, sqlx::Error> {
        private_members::insert_circle_member_inner(conn, circle_group_id, new_member_id).await
    }
}

#[async_trait]
pub trait QueryCircleFn {
    async fn query_circle(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<CircleGroupWithProfileQueryResult>, sqlx::Error>;
}

#[async_trait]
impl QueryCircleFn for DbRepo {
    async fn query_circle(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<CircleGroupWithProfileQueryResult>, sqlx::Error> {
        private_members::query_circle_inner(conn, id).await
    }
}

#[async_trait]
pub trait QueryCircleMemberFn {
    async fn query_circle_member(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<CircleGroupMemberWithProfileQueryResult>, sqlx::Error>;
}

#[async_trait]
impl QueryCircleMemberFn for DbRepo {
    async fn query_circle_member(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<CircleGroupMemberWithProfileQueryResult>, sqlx::Error> {
        private_members::query_circle_member_inner(conn, id).await
    }
}

#[cfg(test)]
mod tests {
    use crate::common::entities::circle_group::model::{ CircleGroupMemberWithProfileQueryResult, CircleGroupWithProfileQueryResult};
    use crate::{common_tests::actix_fixture::get_conn_pool, common::entities::profiles::{repo::{InsertProfileFn, QueryProfileFn}, model::ProfileCreate}};
    use super::*;
    use super::{InsertCircleFn, private_members};
    use crate::common::entities::profiles::model::ProfileQueryResult;
    use std::sync::{ Arc, RwLock };
    use lazy_static::lazy_static;

    #[derive(Clone)]
    #[allow(unused)]
    struct Fixtures {
        pub follower: ProfileQueryResult,
        pub following_profiles: Vec<ProfileQueryResult>,
        pub circle_group: CircleGroupWithProfileQueryResult,
        pub circle_group_members: Vec<CircleGroupMemberWithProfileQueryResult>,
        pub conn: Pool<Postgres>
    }

    const PREFIX: &str = "Test circle";
    
    async fn setup_data(conn: Pool<Postgres>) -> Fixtures {
        let db_repo = DbRepo;

        let follower_result_id = db_repo.insert_profile(&conn, ProfileCreate { 
            user_name: "follower".to_string(), 
            full_name: "Follower Guy".to_string(), 
            description: format!("{} Follower's description", PREFIX), 
            region: Some("usa".to_string()), 
            main_url: Some("http://whatever.com".to_string()), 
            avatar: vec![] 
        }).await;
        let follower = match follower_result_id {
            Ok(id) => {
                let profile_result = db_repo.query_profile(&conn, id).await;
                match profile_result {
                    Ok(profile) => profile,
                    Err(_) => None
                }
            },
            Err(_) => None
        }.unwrap();

        let mut following_profiles = Vec::new();
        for _ in [..11] {               
            let following_result_id = db_repo.insert_profile(&conn, ProfileCreate { 
                user_name: "following".to_string(), 
                full_name: "Following Guy".to_string(), 
                description: format!("{} Following's description", PREFIX), 
                region: Some("usa".to_string()), 
                main_url: Some("http://whatever.com".to_string()), 
                avatar: vec![] 
            }).await;

            let following = match following_result_id {
                Ok(id) => {
                    let profile_result = db_repo.query_profile(&conn, id).await;
                    match profile_result {
                        Ok(profile) => profile,
                        Err(_) => None
                    }
                },
                Err(_) => None
            }.unwrap();

            following_profiles.push(following);
        }

        let follower_id = follower.id;
        let insert_circle_result_id = db_repo.insert_circle(&conn, follower_id).await.unwrap();
        let circle_group = db_repo.query_circle(&conn, insert_circle_result_id).await.unwrap().unwrap();
        
        let mut circle_group_members = Vec::new();
        for _ in [..11] {
            let current_following = following_profiles.iter().nth(0).unwrap();
            let insert_circle_member_id = db_repo.insert_circle_member(&conn, circle_group.id, current_following.id).await.unwrap();
            let circle_member = db_repo.query_circle_member(&conn, insert_circle_member_id).await.unwrap().unwrap();
            circle_group_members.push(circle_member);
        }

        Fixtures {
            follower,
            following_profiles,
            circle_group,
            circle_group_members,
            conn
        }
    }

    lazy_static!{
        static ref FIXTURES: Arc<RwLock<Option<Fixtures>>> = Arc::new(RwLock::new(None));
    }    

    async fn setup_fixtures() {        
        let fixtures = Arc::clone(&FIXTURES);
        let mut writeable_fixtures = fixtures.write().expect("Failed to get write lock on CircleRepo");
 
        match writeable_fixtures.clone() {            
            Some(_) => (),
            None => {
                println!("log: start circle setup_fixtures()");
                let conn = get_conn_pool().await;
                
                *writeable_fixtures = Some(setup_data(conn).await);
                println!("log: end circle setup_fixtures()");
            }
        };        
    }    

    fn fixtures() -> Fixtures {
        Arc::clone(&FIXTURES).read().unwrap().clone().unwrap()
    }

    mod test_mod_insert_new_circle_group {           
        use super::*;
        struct CircleRepo;

        #[async_trait]
        impl InsertProfileFn for CircleRepo {
            async fn insert_profile(&self, _: &Pool<Postgres>, _: ProfileCreate) -> Result<i64, sqlx::Error> {
                Ok(fixtures().follower.id)
            }
        }

        #[async_trait]
        impl QueryProfileFn for CircleRepo {
            async fn query_profile(&self, _: &Pool<Postgres>, _: i64) -> Result<Option<ProfileQueryResult>, sqlx::Error> {
                Ok(Some(fixtures().follower))
            }
        }

        #[async_trait]
        impl InsertCircleFn for CircleRepo {
            async fn insert_circle(&self, conn: &Pool<Postgres>, circle_owner_id: i64) -> Result<i64, sqlx::Error> {
                private_members::insert_circle_inner(conn, circle_owner_id).await
            }
        }

        #[tokio::test] // every instance of this macro creates a new tokio runtime!
        async fn test_insert_new_circle_group () {
            setup_fixtures().await;
            let db_repo = CircleRepo;
            let fixtures = fixtures();

            let profile_id = db_repo.insert_profile(&fixtures.conn, ProfileCreate { 
                user_name: "follower".to_string(), 
                full_name: "Follower Guy".to_string(), 
                description: "Follower's description".to_string(), 
                region: Some("usa".to_string()), 
                main_url: Some("http://whatever.com".to_string()), 
                avatar: vec![] 
            }).await.unwrap();
            let profile = db_repo.query_profile(&fixtures.conn, profile_id).await.unwrap().unwrap();

            let circle_id = db_repo.insert_circle(&fixtures.conn, profile.id).await.unwrap();

            assert!(circle_id > 0);
        }
    }

    mod test_mod_insert_new_circle_member {
        use super::*;

        struct CircleMemberRepo;

        #[async_trait]
        impl InsertProfileFn for CircleMemberRepo {
            async fn insert_profile(&self, _: &Pool<Postgres>, params: ProfileCreate) -> Result<i64, sqlx::Error> {
                if fixtures().follower.user_name == params.user_name {
                    Ok(fixtures().follower.id)
                } else {
                    Ok(fixtures().following_profiles.iter().find(|p| {
                        p.user_name == params.user_name
                    }).unwrap().id)
                }                
            }
        }

        #[async_trait]
        impl InsertCircleFn for CircleMemberRepo {
            async fn insert_circle(&self, _: &Pool<Postgres>, _: i64) -> Result<i64, sqlx::Error> {
                Ok(fixtures().circle_group.id)
            }
        }

        #[async_trait]
        impl InsertCircleMemberFn for CircleMemberRepo {
            async fn insert_circle_member(&self, conn: &Pool<Postgres>, circle_group_id: i64, new_member_id: i64) -> Result<i64, sqlx::Error> {
                private_members::insert_circle_member_inner(conn, circle_group_id, new_member_id).await
            }
        }

        #[async_trait]
        impl QueryCircleMemberFn for CircleMemberRepo {
            async fn query_circle_member(&self, conn: &Pool<Postgres>, id: i64) -> Result<Option<CircleGroupMemberWithProfileQueryResult>, sqlx::Error> {
                private_members::query_circle_member_inner(conn, id).await
            }
        }

        #[tokio::test]
        async fn test_insert_new_circle_group_member () {
            setup_fixtures().await;
            let db_repo = CircleMemberRepo;
            let fixtures = fixtures();

            let follower_id = db_repo.insert_profile(&fixtures.conn, ProfileCreate { 
                user_name: "follower".to_string(), 
                full_name: "Follower Guy".to_string(), 
                description: "Follower's description".to_string(), 
                region: Some("usa".to_string()), 
                main_url: Some("http://whatever.com".to_string()), 
                avatar: vec![] 
            }).await.unwrap();
            let circle_group_id = db_repo.insert_circle(&fixtures.conn, follower_id).await.unwrap();

            let following_id = db_repo.insert_profile(&fixtures.conn, ProfileCreate { 
                user_name: "following".to_string(), 
                full_name: "following Guy".to_string(), 
                description: "following's description".to_string(), 
                region: Some("usa".to_string()), 
                main_url: Some("http://whatever.com".to_string()), 
                avatar: vec![] 
            }).await.unwrap();

            let circle_member_id = db_repo.insert_circle_member(&fixtures.conn, circle_group_id, following_id).await.unwrap();

            assert!(circle_member_id > 0);
        }

        #[tokio::test]
        async fn test_insert_new_circle_group_member_and_verify_fields () {
            setup_fixtures().await;
            let db_repo = CircleMemberRepo;
            let fixtures = fixtures();

            let follower_id = db_repo.insert_profile(&fixtures.conn, ProfileCreate { 
                user_name: "follower".to_string(), 
                full_name: "Follower Guy".to_string(), 
                description: "Follower's description".to_string(), 
                region: Some("usa".to_string()), 
                main_url: Some("http://whatever.com".to_string()), 
                avatar: vec![] 
            }).await.unwrap();
            let circle_group_id = db_repo.insert_circle(&fixtures.conn, follower_id).await.unwrap();

            let following_id = db_repo.insert_profile(&fixtures.conn, ProfileCreate { 
                user_name: "following".to_string(), 
                full_name: "following Guy".to_string(), 
                description: "following's description".to_string(), 
                region: Some("usa".to_string()), 
                main_url: Some("http://whatever.com".to_string()), 
                avatar: vec![] 
            }).await.unwrap();

            let circle_member_id = db_repo.insert_circle_member(&fixtures.conn, circle_group_id, following_id).await.unwrap();
            let circle_member = db_repo.query_circle_member(&fixtures.conn, circle_member_id).await.unwrap().unwrap();

            assert!(circle_member_id > 0);
            assert!(circle_member.id == circle_member_id);
            assert!(circle_member.circle_group_id == circle_group_id);
            assert!(circle_member.member_id == following_id);
        }
    }
}