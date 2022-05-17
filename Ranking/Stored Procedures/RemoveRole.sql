
CREATE PROCEDURE [Ranking].[RemoveRole]
	@serverid decimal(20, 0),
	@roleid decimal(20, 0)
AS
BEGIN
	SET NOCOUNT ON;
	SET XACT_ABORT ON;

    BEGIN TRAN

	DELETE FROM [Ranking].[Role] WHERE role_id = @roleid;

	IF @@ROWCOUNT > 0
		SELECT CAST(1 AS BIT)
	ELSE
		SELECT CAST(0 AS BIT)

    COMMIT
END
