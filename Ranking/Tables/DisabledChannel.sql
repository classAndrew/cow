CREATE TABLE [Ranking].[DisabledChannel] (
    [id]         INT          IDENTITY (1, 1) NOT NULL,
    [server_id]  DECIMAL (20) NOT NULL,
    [channel_id] DECIMAL (20) NOT NULL,
    CONSTRAINT [PK_DisabledChannel] PRIMARY KEY CLUSTERED ([id] ASC),
    CONSTRAINT [FK_DisabledChannel_Server] FOREIGN KEY ([server_id]) REFERENCES [Ranking].[Server] ([id])
);


GO
CREATE UNIQUE NONCLUSTERED INDEX [IX_DisabledChannel]
    ON [Ranking].[DisabledChannel]([channel_id] ASC);

