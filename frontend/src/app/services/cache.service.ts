import { HttpClient } from '@angular/common/http';
import { of, Observable } from 'rxjs';
import { Injectable } from '@angular/core';
import { shareReplay } from 'rxjs/operators';

@Injectable({
  providedIn: 'root',
})

export class CacheService {

  private cache: any = {};
  private defaultKeepFor: number = 1000 * 60 * 5;

  constructor(private http: HttpClient) {}

  getCached<T>(key: string, clear?: boolean): Observable<T> {
    if (this.cache[key]) {
      if (this.cache[key].isOutdated() || clear) {
        delete this.cache[key];
      } else {
        return this.cache[key].data;
      }
    }
    this.cache[key] = new CacheEntry(this.http.get<T>(key).pipe(shareReplay(1)), this.defaultKeepFor);
    return this.cache[key].data;
  }

  clearCache(key?: string): void {
    if (key) {
      delete this.cache[key];
    } else {
      this.cache = [];
    }
  }
}

class CacheEntry {

  public data: Observable<any>;
  private time: number;
  private keepUntil: number;

  constructor(data: any, keepFor: number) {
    this.data = data;
    this.time = new Date().getTime();
    this.keepUntil = this.time + keepFor;
  }

  isOutdated(): boolean {
    return new Date().getTime() > this.keepUntil;
  }
}
