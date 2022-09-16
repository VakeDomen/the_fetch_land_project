import { Component, Input, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { CardSale } from 'src/app/models/card-sale.model';
import { UserCredentials } from 'src/app/models/user-credentials.model';
import { AuthService } from 'src/app/services/auth.service';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-sale-card',
  templateUrl: './sale-card.component.html',
  styleUrls: ['./sale-card.component.sass']
})
export class SaleCardComponent implements OnInit {

  @Input() cardSale: CardSale | undefined;
  public sellerCredentials: UserCredentials | undefined;

  constructor(
    private data: DataService,
    private auth: AuthService,
  ) { }

  ngOnInit(): void {
    if (this.auth.isLoggedIn() && this.cardSale) {
      this.data.getUserCredentials(this.cardSale.user_id).subscribe((creds: UserCredentials) => this.sellerCredentials = creds);
    }
  }

  public isLoggedIn(): boolean {
    return this.auth.isLoggedIn();
  }

  public shortenWebUrl(url: string): string {
    return `${url.substring(0, 25)}${ url.length > 25 ? '...' : ''}`;
  }

  public routeToLogin() {
    document.location = this.auth.getGoogleLoginUrl()
  }
}
