# Ohjelmakartta Rustilla ja Reactilla

-   backend: rust (container)

    -   toimintalogiikka

        -   ohjelmatiedot säilytetään RAM-muistissa, välitön vastaus

        -   ohjelmatietojen muuttaminen tietokantamuotoon
            (https://github.com/actix/examples/tree/master/databases/sqlite)

        -   ohjelmatietojen kaappaaminen

        -   optio: lisätiedot

            -   ohjelmantunnistus (nimi, kanavat)
            -   uutiset, areena-id, wikipedia, imdb, katsojaprofiili

        -   prosessointi

            -   ohjelman toistuminen
            -   viikon ohjelmakartta häviöllisesti esitettynä

    -   komentoriviliittymä

        -   add \[xmlfile\]
        -   state
        -   remove 123
        -   list -t film

    -   REST-palvelin (Actix Web, https://actix.rs/docs/)

        -   tietojen hallinta (authorization)

            -   syöttäminen

                -   POST /manage/add?type=text, syöttö
                    tekstimuodossa
                -   POST /manage/add?type=xmltv, syöttö
                    xmltv-muodossa
                    (<https://crates.io/crates/xmltv>,
                    <https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html>)
                -   POST /manage/add?type=cron, syöttö cron-muodossa
                    (<https://crates.io/crates/cron>)
                -   Optio: POST /manage/grab, kaappaajan lisääminen
                -   palauttaa lisäystunnuksen, statistiikkaa ja
                    ilmoituksen ristiriidasta

            -   tarkastus

                -   GET /manage/addings
                -   GET /manage/state

            -   poisto lisäystunnukssella

                -   DELETE */manage?id=123*

        -   saanti (käyttää toimintalogiikkaa)

            -   -   *GET /programs *(oletuksena päivän kaikki
                    ohjelmat)
                -   GET /programs?title=pitaako-olla-huolissaan
                -   GET /programs?type=film
                -   GET /programs?coming=4
                -   GET /programs?start=20&stop=23
                -   GET /programs?channel=mtv3
                -   myös atom/rss-syöte

-   frontend: JS, React (container)

    -   erilaisia näkymiä
    -   tietojen syöttö, seuranta ja poisto salasanalla
    -   optio: katsojaprofiilit
    -   eilisen ääni muisteltiin kaarina alava artturi elmen
        muistettiin luvialla merikapteenine häitä
