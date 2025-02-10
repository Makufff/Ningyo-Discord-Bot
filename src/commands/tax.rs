use async_trait::async_trait;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::command::CommandOptionType;
use serenity::builder::CreateApplicationCommand;
use serenity::prelude::*;
use super::command_handler::SlashCommand;

pub struct TaxCommand;

fn calculate_tax(income: f64) -> f64 {
    match income {
        i if i <= 150000.0 => 0.0,
        i if i <= 300000.0 => (i - 150000.0) * 0.05,
        i if i <= 500000.0 => 7500.0 + (i - 300000.0) * 0.10,
        i if i <= 750000.0 => 27500.0 + (i - 500000.0) * 0.15,
        i if i <= 1000000.0 => 65000.0 + (i - 750000.0) * 0.20,
        i if i <= 2000000.0 => 115000.0 + (i - 1000000.0) * 0.25,
        i if i <= 5000000.0 => 365000.0 + (i - 2000000.0) * 0.30,
        i => 1265000.0 + (i - 5000000.0) * 0.35,
    }
}

fn format_number(num: f64) -> String {
    let whole = num.trunc() as i64;
    let decimal = (num.fract() * 100.0).round() as i64;
    
    let mut result = String::new();
    let whole_str = whole.to_string();
    let len = whole_str.len();
    
    for (i, c) in whole_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    
    if decimal > 0 {
        format!("{}.{:02}", result, decimal)
    } else {
        result
    }
}

#[async_trait]
impl SlashCommand for TaxCommand {
    fn name(&self) -> &'static str {
        "tax"
    }

    fn description(&self) -> &'static str {
        "คำนวณภาษีเงินได้บุคคลธรรมดา"
    }

    async fn run(&self, ctx: Context, command: ApplicationCommandInteraction) -> Result<(), String> {
        let options = &command.data.options;
        
        // Get income from command options with better error handling
        let income = match options.first() {
            Some(opt) => match opt.value.as_ref() {
                Some(val) => val.as_f64().ok_or("Invalid number format")?,
                None => return Err("No value provided".to_string()),
            },
            None => return Err("Please provide an income amount".to_string()),
        };

        // Validate income
        if income < 0.0 {
            return Err("Income cannot be negative".to_string());
        }

        let tax = calculate_tax(income);

        let content = format!(
            "💰 **ผลการคำนวณภาษี**\n\
            รายได้: {} บาท\n\
            ภาษีที่ต้องจ่าย: {} บาท",
            format_number(income),
            format_number(tax)
        );

        command
            .create_interaction_response(&ctx.http, |r| {
                r.interaction_response_data(|message| 
                    message.content(content)
                )
            })
            .await
            .map_err(|e| format!("Error sending response: {e:?}"))
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command
            .name(self.name())
            .description(self.description())
            .create_option(|option| {
                option
                    .name("income")
                    .description("รายได้ต่อปี (บาท)")
                    .kind(CommandOptionType::Number)
                    .required(true)
            })
    }
} 