CREATE TABLE [Ranking].[Level] (
    [id]           INT           IDENTITY (1, 1) NOT NULL,
    [server_id]    DECIMAL (20)  NOT NULL,
    [user_id]      DECIMAL (20)  NOT NULL,
    [xp]           INT           CONSTRAINT [DF_Level_xp] DEFAULT ((0)) NOT NULL,
    [level]        INT           CONSTRAINT [DF_Level_level] DEFAULT ((0)) NOT NULL,
    [last_message] DATETIME2 (7) CONSTRAINT [DF_Level_timeout] DEFAULT (CONVERT([datetime2],'1970-01-01')) NOT NULL,
    CONSTRAINT [PK_Level] PRIMARY KEY CLUSTERED ([id] ASC),
    CONSTRAINT [FK_Level_Server] FOREIGN KEY ([server_id]) REFERENCES [Ranking].[Server] ([id]),
    CONSTRAINT [FK_Level_User] FOREIGN KEY ([user_id]) REFERENCES [Ranking].[User] ([id])
);

