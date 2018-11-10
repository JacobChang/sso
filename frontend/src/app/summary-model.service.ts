import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { session } from "./model";

export interface SummaryQuota {
  total: number;
  used: number;
}

export interface Summary {
  applications: SummaryQuota;
  authorizations: SummaryQuota;
  contacts: SummaryQuota;
}

export interface SummaryStore {
  summary?: Summary;
}

@Injectable({
  providedIn: "root"
})
export class SummaryModelService {
  private store: SummaryStore = {
    summary: null
  };
  private subject: BehaviorSubject<Summary> = new BehaviorSubject(null);

  constructor(private http: HttpClient) {}

  get summary() {
    return this.subject.asObservable();
  }

  select() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json",
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/summary";
    this.http.get(apiUri, options).subscribe((summary: Summary) => {
      this.store.summary = summary;
      this.subject.next(summary);
    });
  }
}
