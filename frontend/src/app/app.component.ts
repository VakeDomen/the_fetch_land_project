import { Component, Inject, PLATFORM_ID } from '@angular/core';
import { NavigationEnd, Router } from '@angular/router';
import { BehaviorSubject, filter, Subscription } from 'rxjs';
import {
  NgcCookieConsentService,
  NgcNoCookieLawEvent,
  NgcInitializeEvent,
  NgcStatusChangeEvent,
} from "ngx-cookieconsent";
import { isPlatformBrowser } from '@angular/common';


declare const gtag: Function;


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent {
  title = 'The_Fetch_Land_Project';
  private subbed: boolean = false;
  // SSR
  static isBrowser = new BehaviorSubject<boolean>(false);

  //keep refs to subscriptions to be able to unsubscribe later
  private popupOpenSubscription: Subscription | undefined;
  private popupCloseSubscription: Subscription | undefined;
  private initializeSubscription: Subscription | undefined;
  private statusChangeSubscription: Subscription | undefined;
  private revokeChoiceSubscription: Subscription | undefined;
  private noCookieLawSubscription: Subscription | undefined;

  constructor(
    private ccService: NgcCookieConsentService,
    private router: Router,
    @Inject(PLATFORM_ID) private platformId: any
  ) {
    AppComponent.isBrowser.next(isPlatformBrowser(platformId));
    if (isPlatformBrowser(this.platformId)) {
      this.router.events.subscribe((event) => {
        if (event instanceof NavigationEnd) {
          gtag('config', 'MEASUREMENT-ID', { 'page_path': event.urlAfterRedirects });
        }      
      })
    }
  }


  ngOnInit() {
    this.popupOpenSubscription = this.ccService.popupOpen$.subscribe(
      () => {}
    );

    this.popupCloseSubscription = this.ccService.popupClose$.subscribe(
      () => {}
    );

    this.initializeSubscription = this.ccService.initialize$.subscribe(
      (event: NgcInitializeEvent) => {}
    );

    this.statusChangeSubscription = this.ccService.statusChange$.subscribe(
      (event: NgcStatusChangeEvent) => {
        if (event.status == 'allow') {
          this.subbed = true;
          this.router.events
          .pipe(
            filter((event): event is NavigationEnd => event instanceof NavigationEnd)
          ).subscribe((event: NavigationEnd) => {
            /** START : Code to Track Page View  */
            if (this.subbed) {
              gtag('event', 'page_view', {
                 page_path: event.urlAfterRedirects
              });
            }
            /** END */
          })
        } else {
          this.subbed = false;
        }
      }
    );

    this.revokeChoiceSubscription = this.ccService.revokeChoice$.subscribe(
      () => {}
    );

    this.noCookieLawSubscription = this.ccService.noCookieLaw$.subscribe(
      (event: NgcNoCookieLawEvent) => {}
    );
  }

  ngOnDestroy() {
    // unsubscribe to cookieconsent observables to prevent memory leaks
    this.popupOpenSubscription?.unsubscribe();
    this.popupCloseSubscription?.unsubscribe();
    this.initializeSubscription?.unsubscribe();
    this.statusChangeSubscription?.unsubscribe();
    this.revokeChoiceSubscription?.unsubscribe();
    this.noCookieLawSubscription?.unsubscribe();
  }
}
