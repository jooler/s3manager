# Cloudflare R2 Configuration Guide

## Connection Setup

1. Find your Cloudflare R2 S3 API URL:
   - Go to your R2 bucket settings page
   - Look for the "S3 API" section under "Bucket Details"
   - Copy the S3 API URL in the following format:
   ```
   https://[accountId].r2.cloudflarestorage.com/[bucketName]
   ```
   The application will automatically extract the `Bucket Name` and `Account ID` from this URL.

2. Create an API token with the following steps:
   - Visit [Cloudflare R2 API Tokens](https://dash.cloudflare.com/?to=/:account/r2/api-tokens)
   - Generate a new token with at least read and write permissions for objects
   - Copy the "Access Key ID" and paste it into the "Access Key" field
   - Copy the "Secret Access Key" and paste it into the "Secret Key" field

3. (Optional) Configure a Custom Domain:
   - If you have a custom domain for your R2 bucket, enter it in the "Custom Domain" field
   - This domain will be used when generating shareable links for uploaded files

## Verification and Save

1. Click the "Check" button to verify your bucket connectivity
2. If the connection test passes, click "Save" to store your configuration
