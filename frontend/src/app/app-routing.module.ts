import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AdvancedSearchComponent } from './pages/advanced-search/advanced-search.component';
import { ContactComponent } from './pages/contact/contact.component';
import { FrontComponent } from './pages/front/front.component';
import { PrivacyPolicyComponent } from './pages/privacy-policy/privacy-policy.component';
import { ProfileComponent } from './pages/profile/profile.component';
import { RegisterComponent } from './pages/register/register.component';
import { SalesComponent } from './pages/sales/sales.component';
import { TokenComponent } from './pages/token/token.component';
import { AuthGuard } from './services/auth.guard';

const routes: Routes = [
  {
    path: "contact",
    component: ContactComponent,
  },
  {
    path: "privacy",
    component: PrivacyPolicyComponent,
  },
  {
    path: "profile",
    component: ProfileComponent,
    canActivate: [AuthGuard],
  },
  {
    path: "register",
    component: RegisterComponent,
    canActivate: [AuthGuard],
  },
  {
    path: "sales",
    component: SalesComponent,
    canActivate: [AuthGuard],
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
