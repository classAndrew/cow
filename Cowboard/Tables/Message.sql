CREATE TABLE [Cowboard].[Message] (
    [message_id]         DECIMAL (20) NOT NULL,
    [message_channel_id] DECIMAL (20) NOT NULL,
    [post_id]            DECIMAL (20) NOT NULL,
    [post_channel_id]    DECIMAL (20) NOT NULL,
    [guild_id]           DECIMAL (20) NOT NULL,
    CONSTRAINT [PK_Message] PRIMARY KEY CLUSTERED ([message_id] ASC),
    CONSTRAINT [FK_Message_Server] FOREIGN KEY ([guild_id]) REFERENCES [Cowboard].[Server] ([id])
);

