// --- struct definition update ---
#[derive(Serialize, Deserialize, Clone, Debug, Store, Default, Eq, PartialEq)]
struct ResolvedServiceStore {
    #[store(key: String = |rs| rs.instance_fullname.clone())]
    resolved: Vec<ResolvedService>,
    sort_kind: SortKind,
    query: String,
}

// ... later, in Browse() component ...

    let store = Store::new(ResolvedServiceStore::default());

    // Create reactive filtered views
    let filtered_resolved = Memo::new(move |_| {
        let query = store.query().get();
        let resolved = store.resolved().get();
        resolved
            .into_iter()
            .filter(|rs| rs.matches_query(&query))
            .collect::<Vec<_>>()
    });

    let filtered_count = Memo::new(move |_| filtered_resolved.get().len());

    // Create a more efficient filtered iterator that maintains Field references
    let filtered_fields = Memo::new(move |_| {
        let query = store.query().get();
        store.resolved()
            .iter_keyed()
            .filter(|field| field.get().matches_query(&query))
            .collect::<Vec<_>>()
    });

// --- Removed legacy ---
//    let filtered = Store::new(ResolvedServiceStore::default());
//    let query = RwSignal::new(String::new());
//    Effect::watch( ... ) // manual filtering

// ... further down, Badge count UI update ...

    <Badge
        appearance=BadgeAppearance::Tint
        size=BadgeSize::Large
        color=BadgeColor::Subtle
    >
        {move || {
            format!(
                "{}/{}",
                filtered_count.get(),
                store.resolved().read().len(),
            )
        }}
    </Badge>

// ... quick-filter input binding update ...

    <Input
        value=store.query()
        placeholder="Quick filter"
        class=input_class
        on_focus=on_quick_filter_focus
    />

// ... For loop updated to use filtered_fields ...

    <For
        each=move || filtered_fields.get()
        key=move |row| row.get().instance_fullname
        let:resolved_service
    >
        <ResolvedServiceItem resolved_service />
    </For>