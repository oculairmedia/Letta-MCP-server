# 🔒 Fix missing CORS restrictions

🎯 What: Added an allowlist map for Access-Control-Allow-Origin header instead of using a wildcard.
⚠️ Risk: Missing CORS restrictions could allow malicious domains to read sensitive data from the user via the browser.
🛡️ Solution: Created an Nginx map allowing localhost and clearly indicating where production origins should be added. By default, unauthorized domains receive no CORS header.
