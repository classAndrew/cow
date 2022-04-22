use num_traits::ToPrimitive;
use serenity::{
    model::id::{
        UserId
    }
};
use rust_decimal::{
    Decimal,
    prelude::FromPrimitive
};

use crate::Database;
use crate::commands::ucm::courses_db_models::*;

impl Database {
    pub async fn get_user_reminders(&self, user_id: UserId) -> Result<Vec<Reminder>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let user_decimal = Decimal::from_u64(user_id.0).unwrap();
        let res = conn.query(
            "SELECT course_reference_number, min_trigger, for_waitlist, triggered FROM [UniScraper].[UCM].[reminder] WHERE user_id = @P1",
            &[&user_decimal])
            .await?
            .into_first_result()
            .await?;

        let mut out: Vec<Reminder> = Vec::new();

        for reminder in res {
            out.push(Reminder {
                user_id: user_id.0,
                course_reference_number: reminder.get(0).unwrap(),
                min_trigger: reminder.get(1).unwrap(),
                for_waitlist: reminder.get(2).unwrap(),
                triggered: reminder.get(3).unwrap()
            });
        }

        Ok(out)
    }

    pub async fn add_reminder(&self, reminder: &Reminder) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let user_decimal = Decimal::from_u64(reminder.user_id).unwrap();

        // Will panic if there is a duplicate, since I have uniqueness set.
        conn.execute(
            "INSERT INTO [UniScraper].[UCM].[reminder] (user_id, course_reference_number, min_trigger, for_waitlist, triggered) VALUES (@P1, @P2, @P3, @P4, @P5)",
            &[&user_decimal, &reminder.course_reference_number, &reminder.min_trigger, &reminder.for_waitlist, &reminder.triggered])
            .await?;

        Ok(())
    }

    pub async fn remove_reminder(&self, user_id: UserId, course_reference_number: i32) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let user_decimal = Decimal::from_u64(user_id.0).unwrap();

        let total = conn.execute(
            "DELETE FROM [UniScraper].[UCM].[reminder] WHERE user_id = @P1 AND course_reference_number = @P2",
            &[&user_decimal, &course_reference_number])
            .await?.total();

        Ok(total > 0)
    }

    pub async fn trigger_reminders(&self) -> Result<Vec<Trigger>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;

        let res = conn.simple_query(
            "EXEC [UniScraper].[UCM].[TriggerReminders]")
            .await?
            .into_first_result()
            .await?;

        let mut out: Vec<Trigger> = Vec::new();

        for reminder in res {
            let user_id: Decimal = reminder.get(0).unwrap();
            out.push(Trigger {
                user_id: user_id.to_u64().unwrap(),
                course_reference_number: reminder.get(1).unwrap(),
                min_trigger: reminder.get(2).unwrap()
            });
        }

        Ok(out)
    }

    pub async fn get_class(&self, course_reference_number: i32) -> Result<Option<Class>, Box<dyn std::error::Error + Send + Sync>> {
        let mut conn = self.pool.get().await?;
        let res = conn.query(
            "SELECT id, term, course_number, campus_description, course_title, credit_hours, maximum_enrollment, enrollment, seats_available, wait_capacity, wait_available FROM [UniScraper].[UCM].[class] WHERE course_reference_number = @P1",
            &[&course_reference_number])
            .await?
            .into_row()
            .await?;

        let mut out: Option<Class> = None;

        if let Some(class) = res {
            let course_number: &str = class.get(2).unwrap();
            let campus_description: Option<&str> = class.get(3);
            let course_title: Option<&str> = class.get(4);
            out = Some(Class {
                id: class.get(0).unwrap(),
                term: class.get(1).unwrap(),
                course_reference_number,
                course_number: course_number.to_string(),
                campus_description: campus_description.map(|o| o.to_string()),
                course_title: course_title.map(|o| o.to_string()),
                credit_hours: class.get(5).unwrap(),
                maximum_enrollment: class.get(6).unwrap(),
                enrollment: class.get(7).unwrap(),
                seats_available: class.get(8).unwrap(),
                wait_capacity: class.get(9).unwrap(),
                wait_available: class.get(10).unwrap()
            });
        }

        Ok(out)
    }
}