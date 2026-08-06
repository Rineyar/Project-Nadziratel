use serenity::all::{Context, EventHandler, Message};

pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler
{
    //Вызывается при новом сообщении на сервере
    async fn message(&self, _ctx: Context, msg: Message)
    {
        println!("Сообщение от {}: {}", msg.author.name, msg.content);
    }
}