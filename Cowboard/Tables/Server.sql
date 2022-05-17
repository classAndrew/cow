CREATE TABLE [Cowboard].[Server] (
    [id]               DECIMAL (20)  NOT NULL,
    [channel]          DECIMAL (20)  NULL,
    [add_threshold]    INT           CONSTRAINT [DF_Server_add_threshold] DEFAULT ((5)) NOT NULL,
    [remove_threshold] INT           CONSTRAINT [DF_Server_remove_threshold] DEFAULT ((4)) NOT NULL,
    [emote]            NVARCHAR (64) CONSTRAINT [DF_Server_emote] DEFAULT (':cow:') NOT NULL,
    [webhook_id]       DECIMAL (20)  NULL,
    [webhook_token]    VARCHAR (128) NULL,
    CONSTRAINT [PK_Server_1] PRIMARY KEY CLUSTERED ([id] ASC),
    CONSTRAINT [CK_Server_Thresholds_And_Even_More] CHECK ([add_threshold]>=(1)),
    CONSTRAINT [CK_Server_Thresholds_But_More] CHECK ([remove_threshold]>=(0)),
    CONSTRAINT [CK_Servers_Thresholds] CHECK ([add_threshold]>=[remove_threshold])
);

