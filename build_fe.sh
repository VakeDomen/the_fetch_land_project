cd frontend
sudo rm -r node_modules
npx -p @angular/cli@14 npm install
npx -p @angular/cli@14 ng build --configuration production
sudo mv /home/vake/the_fetch_land_project/frontend/dist/the-fetch-land-project /var/www/fetchland.eu
