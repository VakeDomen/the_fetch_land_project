export interface ScryfallResponse<T> {
    object: string,
    has_more: boolean,
    data: T[]
}