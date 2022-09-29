cd frontend
sudo rm -r node_modules
npx -p @angular/cli@14 npm install
npx -p @angular/cli@14 ng build --configuration production
