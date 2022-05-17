CREATE TABLE [Ranking].[Role] (
    [id]        INT           IDENTITY (1, 1) NOT NULL,
    [server_id] DECIMAL (20)  NOT NULL,
    [role_name] NVARCHAR (50) NOT NULL,
    [role_id]   DECIMAL (20)  NULL,
    [min_level] INT           CONSTRAINT [DF_Role_min_level] DEFAULT ((0)) NOT NULL,
    CONSTRAINT [PK_Role] PRIMARY KEY CLUSTERED ([id] ASC),
    CONSTRAINT [FK_Role_Server] FOREIGN KEY ([server_id]) REFERENCES [Ranking].[Server] ([id])
);

