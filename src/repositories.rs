use crate::{models::*, schema::*};
use diesel::{dsl::{now, IntervalDsl}, prelude::*};

pub struct RustaceanRepo;

impl RustaceanRepo {
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .limit(limit)
            .order(rustaceans::id.desc())
            .load::<Rustacean>(c)
    }

    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn create(c: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
    }

    pub fn save(c: &mut PgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }
}

pub struct CrateRepo;

impl CrateRepo {
    pub fn find_since(connection: &mut PgConnection, hours_since: i32) -> QueryResult<Vec<Crate>> {
        crates::table
            .filter(crates::created_at.ge(now - hours_since.seconds()))
            .order(crates::id.desc())
            .load::<Crate>(connection)
    }
    
    pub fn find_multiple(c: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table
            .limit(limit)
            .order(crates::id.desc())
            .load::<Crate>(c)
    }

    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result::<Crate>(c)
    }

    pub fn create(c: &mut PgConnection, new_rustacean: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_rustacean)
            .get_result(c)
    }

    pub fn save(c: &mut PgConnection, id: i32, rustacean: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::name.eq(rustacean.name.to_owned()),
                crates::version.eq(rustacean.version.to_owned()),
                crates::code.eq(rustacean.code.to_owned()),
                crates::description.eq(rustacean.description.to_owned()),
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(c)
    }
}

pub struct UserRepo;

impl UserRepo {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(c)
    }

    pub fn find_by_username(c: &mut PgConnection, username: &str) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result::<User>(c)
    }

    pub fn create(
        c: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let new_user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepo::find_by_code(c, &role_code) {
                    NewUserRole {
                        user_id: new_user.id,
                        role_id: role.id,
                    }
                } else {
                    let name = role_code.as_str().to_owned();
                    let new_role = NewRole {
                        code: role_code,
                        name,
                    };
                    let role = RoleRepo::create(c, new_role)?;
                    NewUserRole {
                        user_id: new_user.id,
                        role_id: role.id,
                    }
                }
            };
            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .execute(c)?;
        }

        Ok(new_user)
    }

    pub fn delete(c: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id))).execute(c)?;
        diesel::delete(users::table.find(id)).execute(c)
    }

    pub fn find_with_roles(
        c: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load(c)?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }
}

pub struct RoleRepo;

impl RoleRepo {
    pub fn find_by_code(c: &mut PgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table
            .filter(roles::code.eq(code))
            .get_result::<Role>(c)
    }

    pub fn find_by_ids(c: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table
            .filter(roles::id.eq_any(ids))
            .get_results::<Role>(c)
    }

    pub fn find_by_user(c: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles: Vec<UserRole> = UserRole::belonging_to(user).get_results(c)?;
        Self::find_by_ids(c, user_roles.iter().map(|x: &UserRole| x.role_id).collect())
    }

    pub fn create(c: &mut PgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
    }
}
