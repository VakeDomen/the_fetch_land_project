import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { Card } from '../models/card.model';
import { NotificationMessage } from '../models/notificationn.model';
import { SaleEdit } from '../models/sale-edit.model';
import { Sale } from '../models/sale.model';
import { ScryfallResponse } from '../models/scryfall-response.model';
import { Set } from '../models/set.model';
import { TrieTree } from '../models/trie.model';
import { UserCredentials } from '../models/user-credentials.model';
import { User } from '../models/user.model';
import { CacheService } from './cache.service';

@Injectable({
  providedIn: 'root'
})
export class DataService {

  private apiUrlUser = environment.api_url + '/user/';
  private apiUrlCard = environment.api_url + '/card/';
  private apiUrlNotification = environment.api_url + '/notify/';

  private cardTrie: TrieTree<Card> = new TrieTree();

  constructor(
    private http: HttpClient,
    private cache: CacheService,
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

  getCardsByPrefix(prefix: string, lang?: string): Observable<Card[]> {
    return this.cache.getCached<Card[]>(`${this.apiUrlCard}name/${lang ?? 'en'}/${prefix}`)
  }

  getSets(): Observable<ScryfallResponse<Set>> {
    return this.cache.getCached<ScryfallResponse<Set>>("https://api.scryfall.com/sets/");
  }

  postSale(sale: Sale): Observable<Sale> {
    return this.http.post<Sale>(`${this.apiUrlUser}sale/`, sale);
  }
  
  postSaleEdit(sid: string, sale: SaleEdit): Observable<Sale> {
    return this.http.patch<Sale>(`${this.apiUrlUser}sale/${sid}`, sale);
  }

  insertCardsToTrie(cards: Card[]) {
    if (!cards) {
      return;
    }
    for (const card of cards) {
      this.cardTrie.insertWord(card.id, card);
    }
  }

  getCardById(id: string): Observable<Card> {
    return this.cache.getCached<Card>(`${this.apiUrlCard}id/${id}`)
  }

  getCardByIdFromTrie(id: string): Card[] {
    return this.cardTrie.collect(id);
  }

  getSalesByPrefix(prefix: string, lang?: string): Observable<Sale[]> {
    return this.cache.getCached<Sale[]>(`${this.apiUrlCard}sales/name/${lang ?? 'en'}/${prefix}`)
  }

  getSalesByPartialPrefix(prefix: string, lang?: string): Observable<Sale[]> {
    return this.cache.getCached<Sale[]>(`${this.apiUrlCard}sales/partials/${lang ?? 'en'}/${prefix}`)
  }
  getUserCredentials(uid: string): Observable<UserCredentials> {
    return this.cache.getCached<UserCredentials>(`${this.apiUrlUser}credentials/${uid}`);
  }

  getLatestSales(num?: number): Observable<Sale[]> {
    return this.cache.getCached<Sale[]>(`${this.apiUrlCard}sales/latest/${num ?? ''}`);
  }

  deleteSale(id: string): Observable<any>  {
    return this.http.delete<any>(`${this.apiUrlUser}sale/${id}`);
  }

  getAmountOfSales(): Observable<number> {
    return this.cache.getCached<number>(`${this.apiUrlCard}sales/num/`);
  }

  getSalesPaged(page_size: number, page_offset: number, column_sort: string): Observable<Sale[]> {
    return this.cache.getCached<Sale[]>(`${this.apiUrlCard}sales/paged/${page_size}/${page_offset}/${column_sort}`);
  }

  sendNotification(notification: NotificationMessage): Observable<undefined> {
    return this.http.post<undefined>(this.apiUrlNotification, notification)
  }
}
