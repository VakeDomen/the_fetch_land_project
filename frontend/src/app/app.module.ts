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
import { FormsModule } from '@angular/forms';
import { ContactInfoFormComponent } from './components/contact-info-form/contact-info-form.component';
import { CardSelectComponent } from './components/card-select/card-select.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { CardTableComponent } from './components/card-table/card-table.component';
import { MyOfferListComponent } from './components/my-offer-list/my-offer-list.component';
import { NewSaleDetailsComponent } from './components/new-sale-details/new-sale-details.component';
import { MyOffersTableComponent } from './components/my-offers-table/my-offers-table.component';
import { SaleSearchTableComponent } from './components/sale-search-table/sale-search-table.component';
import { SaleSearchComponent } from './components/sale-search/sale-search.component';
import { SaleListComponent } from './components/sale-list/sale-list.component';


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
    SalesComponent,
    ContactInfoFormComponent,
    CardSelectComponent,
    CardTableComponent,
    MyOfferListComponent,
    NewSaleDetailsComponent,
    MyOffersTableComponent,
    SaleSearchTableComponent,
    SaleSearchComponent,
    SaleListComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    HttpClientModule,
    FormsModule,
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
