
CREATE PROCEDURE [Ranking].[AddUser]
	@serverid decimal(20, 0),
	@userid decimal(20, 0)
AS
BEGIN
	SET NOCOUNT ON;
	SET XACT_ABORT ON;

	BEGIN TRAN

	EXEC [Ranking].[AddServer] @serverid = @serverid;

	MERGE INTO [Ranking].[User] WITH (HOLDLOCK) AS Target
    USING (Values(@userid)) AS SOURCE([user_id])
    ON Target.id = @userid
    WHEN NOT MATCHED THEN
    INSERT (id) VALUES (@userid);

	MERGE INTO [Ranking].[Level] WITH (HOLDLOCK) AS Target
    USING (Values(@serverid, @userid)) AS SOURCE(server_id, [user_id])
    ON Target.server_id = @serverid AND Target.[user_id] = @userid
    WHEN NOT MATCHED THEN
    INSERT (server_id, [user_id]) VALUES (@serverid, @userid);
	
	COMMIT
END
