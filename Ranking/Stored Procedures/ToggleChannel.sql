
CREATE PROCEDURE [Ranking].[ToggleChannel]
	@serverid decimal(20, 0),
	@channelid decimal(20, 0)
AS
BEGIN
	SET NOCOUNT ON;
	SET XACT_ABORT ON;

	EXEC [Ranking].[AddServer] @serverid = @serverid;

    MERGE INTO [Ranking].[DisabledChannel] WITH (HOLDLOCK) AS Target
    USING (Values(@serverid)) AS SOURCE(server_id)
    ON Target.server_id = @serverid AND Target.channel_id = @channelid
	WHEN MATCHED THEN
	DELETE
    WHEN NOT MATCHED THEN
    INSERT (server_id, channel_id) VALUES (@serverid, @channelid);

	SELECT CAST(1 AS BIT) FROM [Ranking].[DisabledChannel] WHERE server_id = @serverid AND channel_id = @channelid;
END
