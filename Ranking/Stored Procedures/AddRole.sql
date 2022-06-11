
CREATE PROCEDURE [Ranking].[AddRole]
    @server_id decimal(20, 0),
    @role_name nvarchar(MAX),
    @role_id decimal(20, 0),
    @min_level decimal(20, 0)
AS
BEGIN
    SET NOCOUNT ON;
    SET XACT_ABORT ON;

	IF EXISTS (SELECT 1 FROM [Ranking].[Role] WHERE server_id = @server_id AND min_level = @min_level)
	BEGIN
		SELECT CAST(0 AS BIT)
		RETURN
	END

    BEGIN TRAN

    MERGE INTO [Ranking].[Role] WITH (HOLDLOCK) AS Target
    USING (Values(@role_id)) AS SOURCE([role_id])
    ON Target.role_id = @role_id
    WHEN MATCHED THEN
    UPDATE SET server_id=@server_id, role_name=@role_name, role_id=@role_id, min_level=@min_level
    WHEN NOT MATCHED THEN
    INSERT (server_id, role_name, role_id, min_level) VALUES (@server_id, @role_name, @role_id, @min_level);
    
    COMMIT

	SELECT CAST(1 AS BIT)
END