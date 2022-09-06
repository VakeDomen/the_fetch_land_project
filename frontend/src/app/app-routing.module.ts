import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AdvancedSearchComponent } from './pages/advanced-search/advanced-search.component';
import { ContactComponent } from './pages/contact/contact.component';
import { FrontComponent } from './pages/front/front.component';
import { ProfileComponent } from './pages/profile/profile.component';
import { RegisterComponent } from './pages/register/register.component';
import { SaleComponent } from './pages/sale/sale.component';
import { TokenComponent } from './pages/token/token.component';

const routes: Routes = [
  {
    path: "contact",
    component: ContactComponent,
  },
  {
    path: "profile",
    component: ProfileComponent,
  },
  {
    path: "register",
    component: RegisterComponent,
  },
  {
    path: "sale/:id",
    component: SaleComponent,
  },
  {
    path: "search",
    component: AdvancedSearchComponent,
  },
  {
    path: "token/:token",
    component: TokenComponent,
  },
  {
    path: "**",
    component: FrontComponent,
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
