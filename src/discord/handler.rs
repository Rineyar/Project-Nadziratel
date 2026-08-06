use serenity::all::{Context, EventHandler, Message, ChannelId}; //Для Handler
use tokio::sync::mpsc::{Sender}; //Связь с распределителем

pub struct Handler
{
    pub img_tx: Sender<(Vec<u8>, ChannelId)>,
}

#[serenity::async_trait]
impl EventHandler for Handler
{
    //Вызывается при новом сообщении на сервере
    async fn message(&self, _ctx: Context, msg: Message)
    {
        //println!("Сообщение от {}: {}", msg.author.name, msg.content);

        if msg.attachments.is_empty()
        {
            //println!("Вложений не содержит");
        }

        for attachment in msg.attachments.iter()
        {
            let is_image: bool = attachment.content_type.as_deref().is_some_and(|content_type| content_type.starts_with("image/"));

            if !is_image
            {
                //println!("Не изображение");

                continue;
            }

            match attachment.download().await
            {
                Ok(bytes) =>
                {
                    if self.img_tx.send((bytes, msg.channel_id)).await.is_err()
                    {
                        eprintln!("Image channel closed");
                    }
                }

                Err(err) =>
                {
                    eprintln!("Download error!\n{:?}", err);
                }
            }
        }
    }
}