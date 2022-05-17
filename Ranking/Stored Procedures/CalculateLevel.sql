
CREATE PROCEDURE [Ranking].[CalculateLevel]
	@level int,
	@xp int = NULL OUTPUT
AS
BEGIN
	SET NOCOUNT ON;
	SET XACT_ABORT ON;

	SET @xp = CEILING(5.85224 * @level * @level + 238.332 * @level - 48.3852);

	IF @xp < 100
		SET @xp = 100;

	SELECT @xp;

	RETURN;
END
