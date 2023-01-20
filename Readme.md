# WebPy

This project is still in progress

# Execution of pre-packaged build

0. Make sure Python is installed and is executable via 'python'. In theory all 3.x versions of pyhton should be fine. We used Python 3.10.4 .

1. Unzip 'pre_packaged_env.zip'

2. Open the Powershell

3. execute the server binary: '.\target\debug\webserver.exe' Note: it should be executed at the root of the unzipped directory

4. In a browser (we used Firefox) open 'localhost:3000'

5. The fetches and serves the appropriate files now

6. Note: for testing purposes, the python runtime searches for a function named 'hello'. If it cannot be found in the user code in the webclient, an error is raised. Please define one.
