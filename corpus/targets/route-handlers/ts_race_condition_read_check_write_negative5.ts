// Negative example: reading a value, checking it via ternary, and returning without writing state
export function retrieveAppVersion(config: any, utils: any) {
  return (req: any, res: any) => {
    // This is safe because there is no DB write, just returning data
    res.json({
      version: config.get('application.showVersionNumber') ? utils.version() : ''
    });
  }
}

export function logData(db: any) {
    const data = db.read("some_id");
    if (data) {
        console.log(data.getValue());
    }
}
