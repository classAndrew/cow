
CREATE PROCEDURE [Ranking].[AddServer]
	@serverid decimal(20, 0)
AS
BEGIN
	SET NOCOUNT ON;
	SET XACT_ABORT ON;

	BEGIN TRAN

	MERGE INTO [Ranking].[Server] WITH (HOLDLOCK) AS Target
    USING (Values(@serverid)) AS SOURCE(server_id)
    ON Target.id = @serverid
    WHEN NOT MATCHED THEN
    INSERT (id) VALUES (@serverid);
	
	COMMIT
END
