import { Component, EventEmitter, Input, OnChanges, OnInit, Output, SimpleChanges } from '@angular/core';
import { Card } from 'src/app/models/card.model';
import { ScryfallResponse } from 'src/app/models/scryfall-response.model';
import { DataService } from 'src/app/services/data.service';
import { Set } from 'src/app/models/set.model';


@Component({
  selector: 'app-card-table',
  templateUrl: './card-table.component.html',
  styleUrls: ['./card-table.component.sass']
})
export class CardTableComponent implements OnChanges {

  @Input() cards: Card[] = [];
  @Input() displayMode: 'new_sale' = 'new_sale';
  @Input() totalCards: number = 0;

  @Output() cardSelected = new EventEmitter<Card>();

  private sets: Set[] = [];

  constructor(
    private data: DataService,
  ) {
    this.data.getSets().subscribe((setsData: ScryfallResponse<Set>) => {
      this.sets = setsData.data;
    })
  }


  ngOnChanges(changes: SimpleChanges): void { }

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

  selectCard(card: Card) {
    this.cardSelected.emit(card);
  }
}
