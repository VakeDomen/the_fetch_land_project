import { SafeCall } from '@angular/compiler';
import { Component, OnInit } from '@angular/core';
import { CardSale } from 'src/app/models/card-sale.model';
import { Card } from 'src/app/models/card.model';
import { Sale } from 'src/app/models/sale.model';
import { DataService } from 'src/app/services/data.service';
import { Set } from 'src/app/models/set.model';
import { ScryfallResponse } from 'src/app/models/scryfall-response.model';

@Component({
  selector: 'app-new-sales-table',
  templateUrl: './new-sales-table.component.html',
  styleUrls: ['./new-sales-table.component.sass']
})
export class NewSalesTableComponent implements OnInit {

  public cardSales: CardSale[] = [];
  public cards: Card[] = [];
  public sets: Set[] = []

  
  constructor(
    private data: DataService,
  ) { }

  ngOnInit(): void {
    this.data.getLatestSales(5).subscribe((sales: Sale[]) => this.fillTrieSetup(sales))
    this.data.getSets().subscribe((setsData: ScryfallResponse<Set>) => {this.sets = setsData.data})
  }


  private async fillTrieSetup(sales: Sale[]): Promise<void> {
    const cardPromises = sales.map((s: Sale) => {
      const card = this.data.getCardByIdFromTrie(s.sale_object_id);
      if (card.length) {
        return new Promise<Card>((resolve) => resolve(card.pop() as Card));
      } else {
        return this.data.getCardById(s.sale_object_id).toPromise() as Promise<Card>;
      }
    });
    const cards = await Promise.all(cardPromises);
    this.cards = cards;
    this.cards.map((c: Card) => this.data.insertCardsToTrie([c]));
    this.cardSales = sales.map((sale: Sale) => {
      const card = this.data.getCardByIdFromTrie(sale.sale_object_id).pop() as Card;
      const cs = sale as CardSale;
      cs.card = card;
      return cs;
    });
    this.data.insertCardsToTrie(this.cardSales.map(cs => cs.card));
  }

  public getSetIcon(setId: string): string {
    for (const set of this.sets) {
      if (set.id == setId) return set.icon_svg_uri;
    }
    return '';
  }
}
