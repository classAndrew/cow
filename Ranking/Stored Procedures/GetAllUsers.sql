
CREATE PROCEDURE [Ranking].[GetAllUsers]
	@serverid decimal(20, 0)
AS
BEGIN
	SET NOCOUNT ON;
    SET XACT_ABORT ON;

    CREATE TABLE #User (
		user_id decimal(20, 0),
		[level] int,
		xp int,
		role_id decimal(20, 0)
	);

	DECLARE @id decimal(20, 0) = 0;
	DECLARE @level int = 0;
	DECLARE @xp int = 0;
	DECLARE @roleid decimal(20, 0) = 0;

	WHILE (1 = 1)
	BEGIN
		SELECT TOP 1 @id = user_id, @level = [level], @xp = xp FROM [Ranking].[Level] WHERE server_id = @serverid AND user_id > @id ORDER BY user_id;

		IF @@ROWCOUNT = 0
			BREAK;

		SET @roleid = NULL;
		SELECT TOP 1 @roleid = role_id FROM [Ranking].[Role] WHERE server_id = @serverid AND min_level <= @level ORDER BY min_level DESC;

		INSERT INTO #User (user_id, [level], xp, role_id) VALUES (@id, @level, @xp, @roleid);
	END

	SELECT * FROM #User;
END
