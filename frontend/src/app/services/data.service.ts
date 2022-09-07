import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { Card } from '../models/card.model';
import { Sale } from '../models/sale.model';
import { User } from '../models/user.model';

@Injectable({
  providedIn: 'root'
})
export class DataService {

  private apiUrlUser = environment.api_url + '/user/';
  private apiUrlCard = environment.api_url + '/card/';

  constructor(
    private http: HttpClient,
  ) { }
  
  getUser(): Observable<User> {
    return this.http.get<User>(this.apiUrlUser);
  }

  submitContactInfo(username: string, phone: string): Observable<User> {
    return this.http.patch<User>(this.apiUrlUser, { name: username, phone: phone });
  }

  getUserSales(): Observable<Sale[]> {
    return this.http.get<Sale[]>(this.apiUrlUser + "sales/");
  }

  getCardsByPrefix(prefix: string): Observable<Card[]> {
    return this.http.get<Card[]>(this.apiUrlCard + "name/" + prefix);
  }
}
