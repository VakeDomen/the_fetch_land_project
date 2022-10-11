import { Component, EventEmitter, Input, OnChanges, Output, SimpleChanges } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';
import { CardSale } from 'src/app/models/card-sale.model';
import { Card } from 'src/app/models/card.model';

@Component({
  selector: 'app-sale-list',
  templateUrl: './sale-list.component.html',
  styleUrls: ['./sale-list.component.sass']
})
export class SaleListComponent implements OnChanges {

  @Input() cardSales: CardSale[] = [];
  @Output() back = new EventEmitter();

  public card: Card | undefined;

  constructor(
    private title: Title,
    private meta: Meta,
  ) { }

  ngOnChanges(changes: SimpleChanges): void {
    if (this.cardSales.length) {
      this.card = this.cardSales[0].card;
      const imageLink = this.card.image_uris ? 
        this.card.image_uris.small : 
        (
          this.card.card_faces ? 
            this.card.card_faces[0].image_uris?.small : 
            'assets/logo/fetchland_logo_img.png'
        );
      this.title.setTitle(`${this.card.name} | TheFethclandProject`);
      this.meta.updateTag({name: "description", content: `Pregled oglasov za karto ${this.card.name}.`});
      this.meta.updateTag({name: "og:description", content: `Pregled oglasov za karto ${this.card.name}.`});
      this.meta.updateTag({name: "og:image", content: `${imageLink}`});
    }
  }

  public backTrigger() {
    this.back.emit();
  }

}
