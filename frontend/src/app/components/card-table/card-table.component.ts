import { Component, Input, OnChanges, OnInit, SimpleChanges } from '@angular/core';
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

  private sets: Set[] = [];

  constructor(
    private data: DataService,
  ) { 
    this.data.getSets().subscribe((setsData: ScryfallResponse<Set>) => {
      this.sets = setsData.data;
    })
   }


  ngOnChanges(changes: SimpleChanges): void {
    console.log(this.cards);
    
  }
  
  public getSetIcon(setId: string): string {
    for (const set of this.sets) {
      if (set.id == setId) return set.icon_svg_uri;
    }
    return '';
  }

}
