
CREATE PROCEDURE [Ranking].[SetServerTimeout]
	@serverid decimal(20, 0),
    @timeout decimal(20, 0)
AS
BEGIN
	SET NOCOUNT ON;
	SET XACT_ABORT ON;

	EXEC [Ranking].[AddServer] @serverid = @serverid;

    MERGE INTO [Ranking].[Server] WITH (HOLDLOCK) AS Target
    USING (Values(@serverid)) AS SOURCE(server_id)
    ON Target.id = @serverid
	WHEN MATCHED THEN
	UPDATE SET Target.timeout = @timeout;

	SELECT CAST(1 AS BIT);
END
