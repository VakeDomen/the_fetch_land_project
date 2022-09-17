import { Component, EventEmitter, Input, OnChanges, Output, SimpleChanges } from '@angular/core';
import { CardSale } from 'src/app/models/card-sale.model';
import { ScryfallResponse } from 'src/app/models/scryfall-response.model';
import { DataService } from 'src/app/services/data.service';
import { Set } from 'src/app/models/set.model';
import { Card } from 'src/app/models/card.model';

@Component({
  selector: 'app-sale-search-table',
  templateUrl: './sale-search-table.component.html',
  styleUrls: ['./sale-search-table.component.sass']
})
export class SaleSearchTableComponent implements OnChanges {

  @Input() cardSales: CardSale[] = [];
  @Output() salesToCheck = new EventEmitter<CardSale[]>();

  public cardsOnSale: Card[] = [];
  private sets: Set[] = [];

  constructor(
    private data: DataService,
  ) {
    this.data.getSets().subscribe((setsData: ScryfallResponse<Set>) => {
      this.sets = setsData.data;
    })
  }

  ngOnChanges(changes: SimpleChanges): void {
    // unique card values og cardSales
    console.log(this.cardSales);
    
    this.cardsOnSale = [...new Set(this.cardSales.map(c => c.card))];
  }

  public getSetIcon(setId: string): string {
    for (const set of this.sets) {
      if (set.id == setId) return set.icon_svg_uri;
    }
    return '';
  }

  public mapLanguage(langCode: string): string {
    switch (langCode) {
      case 'en': return 'English'
      case 'es': return 'Spanish'
      case 'fr': return 'French'
      case 'de': return 'German'
      case 'it': return 'Italian '
      case 'pt': return 'Portuguese'
      case 'ja': return 'Japanese'
      case 'ko': return 'Korean'
      case 'ru': return 'Russian'
      case 'zhs': return 'Chinese'
      case 'zht': return 'Chinese'
      case 'he': return 'Hebrew'
      case 'la': return 'Latin'
      case 'grc': return 'Greek'
      case 'ar': return 'Arabic'
      case 'sa': return 'Sanskrit'
      case 'ph': return 'Phyrexian'
      default: return 'Neznan jezik';
    }
  }

  public checkSales(card: Card) {
    this.salesToCheck.emit(this.cardSales.filter(c => c.card.id == card.id));
  }

  public numOfSales(card: Card) {
     return this.cardSales.filter(c => c.card.id == card.id).length;
  }

  public hasSearched() {
    return !!sessionStorage.getItem("saleSearchQeury");
  }
}
