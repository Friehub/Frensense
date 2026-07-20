// SAFE alternative: database triggers prevent deletion
// PostgreSQL example: use row-level security to prevent deletion
/*
CREATE POLICY audit_logs_immutable ON audit_logs
  FOR DELETE
  USING (false);  -- no one can delete

CREATE POLICY audit_logs_read_only ON audit_logs
  FOR UPDATE
  USING (false);  -- no one can update
*/
