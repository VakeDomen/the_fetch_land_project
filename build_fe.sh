cd frontend
sudo rm -r node_modules
npx -p @angular/cli@14 npm install
# -- SSR
npx -p @angular/cli@14 npm run build:ssr
sudo systemctl restart fetchland_web.service

# -- non SSR
#npx -p @angular/cli@14 ng build --configuration production
#sudo mv /var/www/fetchland.eu /var/www/fetchland.eu.old
#sudo mv /home/vake/the_fetch_land_project/frontend/dist/the-fetch-land-project /var/www/fetchland.eu
#sudo rm -r /var/www/fetchland.eu.old
