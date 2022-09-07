import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { FrontComponent } from './pages/front/front.component';
import { ContactComponent } from './pages/contact/contact.component';
import { ProfileComponent } from './pages/profile/profile.component';
import { RegisterComponent } from './pages/register/register.component';
import { SaleComponent } from './pages/sale/sale.component';
import { NavbarComponent } from './components/navbar/navbar.component';
import { AdvancedSearchComponent } from './pages/advanced-search/advanced-search.component';
import { TokenComponent } from './pages/token/token.component';
import { SalesComponent } from './pages/sales/sales.component';
import { HttpClientModule, HTTP_INTERCEPTORS } from '@angular/common/http';
import { AuthInterceptor } from './services/auth.interceptor';

@NgModule({
  declarations: [
    AppComponent,
    FrontComponent,
    ContactComponent,
    ProfileComponent,
    RegisterComponent,
    SaleComponent,
    NavbarComponent,
    AdvancedSearchComponent,
    TokenComponent,
    SalesComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    HttpClientModule,
  ],
  providers: [
    {
      provide : HTTP_INTERCEPTORS,
      useClass: AuthInterceptor,
      multi   : true,
    },
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }
