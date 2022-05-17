CREATE TABLE [Ranking].[Server] (
    [id]      DECIMAL (20) NOT NULL,
    [timeout] INT          CONSTRAINT [DF_Server_timeout] DEFAULT ((60000)) NOT NULL,
    CONSTRAINT [PK_Server] PRIMARY KEY CLUSTERED ([id] ASC)
);

