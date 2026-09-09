# admin_api

Read-only HTTP Lambda for the OWEN admin panel.

## Endpoints

- `GET /messages?status={unprocessed|reserved|processed|rejected}&limit=&cursor=`
- `GET /messages/{messageFolder+}`

Auth is enforced by API Gateway JWT (Cognito). Build with:

```bash
cargo lambda build --release
```
