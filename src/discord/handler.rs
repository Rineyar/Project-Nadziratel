use serenity::all::{AttachmentId, ChannelId, Context, EventHandler, Message, MessageId, Timestamp, UserId}; //Для Handler
use tokio::sync::mpsc::{Sender}; //Связь с распределителем
use tracing::{error};

pub struct Handler
{
    pub img_tx: Sender<Option<(Vec<u8>, MsgData)>>,
}

pub struct MsgData
{
    pub message_id: MessageId,
    pub channel_id: ChannelId,
    pub user_id: UserId,
    pub username: String,
    pub time: Timestamp,
    pub text: String,
    pub att_id: AttachmentId,
    pub filename: String,
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
                    match self.img_tx.send(Some((bytes, MsgData { message_id: msg.id, channel_id: msg.channel_id, user_id: msg.author.id, 
                    username: msg.author.name.clone(), time: msg.timestamp, text: msg.content.clone(), att_id: attachment.id, 
                    filename: attachment.filename.clone() }))).await
                    {
                        Ok(()) => {}
                        
                        Err(err) =>
                        {
                            error!("Img send error!\n{:?}", err);
                        }
                    };
                }

                Err(err) =>
                {
                    error!("Download error!\n{:?}", err);
                }
            }
        }
    }
}