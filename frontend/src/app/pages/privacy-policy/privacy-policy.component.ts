import { animate, style, transition, trigger } from '@angular/animations';
import { Component, Input, OnInit } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';

@Component({
  selector: 'app-privacy-policy',
  templateUrl: './privacy-policy.component.html',
  styleUrls: ['./privacy-policy.component.sass'],
  animations: [
    trigger('enter', [
       transition(':enter', [
        style({ transform: 'translateX(200%)' }),
        animate('200ms'),
      ])
    ]),
  ]  
})
export class PrivacyPolicyComponent implements OnInit {

  @Input() private changeTitle: boolean = false;

  constructor(
    private title: Title,
    private meta: Meta,
  ) { }

  ngOnInit(): void {
    this.title.setTitle("Varovanje osebnih podatkov | TheFethclandProject");
    this.meta.updateTag({name: "description", content: "Opis pristopa vsarovanja osebnih podatkov na strani. "});
  }

}
