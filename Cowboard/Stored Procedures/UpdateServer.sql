
CREATE PROCEDURE [Cowboard].[UpdateServer]
	@id decimal(20, 0),
	@channel decimal(20, 0),
	@add_threshold int,
	@remove_threshold int,
	@emote varchar(64),
	@webhook_id decimal(20, 0),
	@webhook_token varchar(128)
AS
BEGIN
	SET NOCOUNT ON;
	SET XACT_ABORT ON;

	BEGIN TRAN

	MERGE INTO [Cowboard].[Server] WITH (HOLDLOCK) AS Target
    USING (Values(@id)) AS SOURCE([id])
    ON Target.id = @id
	WHEN MATCHED THEN
    UPDATE SET channel = @channel, add_threshold = @add_threshold, remove_threshold = @remove_threshold, emote = @emote, webhook_id = @webhook_id, webhook_token = @webhook_token
    WHEN NOT MATCHED THEN
    INSERT (id, channel, add_threshold, remove_threshold, emote, webhook_id, webhook_token) VALUES (@id, @channel, @add_threshold, @remove_threshold, @emote, @webhook_id, @webhook_token);
	
	COMMIT
END