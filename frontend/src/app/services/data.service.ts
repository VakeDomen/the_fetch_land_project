import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, of } from 'rxjs';
import { environment } from 'src/environments/environment';
import { Card } from '../models/card.model';
import { Sale } from '../models/sale.model';
import { ScryfallResponse } from '../models/scryfall-response.model';
import { Set } from '../models/set.model';
import { TrieTree } from '../models/trie.model';
import { User } from '../models/user.model';
import { CacheService } from './cache.service';

@Injectable({
  providedIn: 'root'
})
export class DataService {

  private apiUrlUser = environment.api_url + '/user/';
  private apiUrlCard = environment.api_url + '/card/';

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
}
