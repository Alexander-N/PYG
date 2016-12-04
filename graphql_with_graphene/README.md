GraphQL with Graphene
=====================

To create an IMDB database see http://imdbpy.sourceforge.net/docs/README.sqldb.txt

* Download the text files from http://www.imdb.com/interfaces
* Download https://github.com/alberanid/imdbpy/blob/master/bin/imdbpy2sql.py
* Execute
    python imdbpy2sql.py -d imdb_data/ -u 'sqlite:///~/graphene/imdbpy/bin/imdb_data/imdb.db' --sqlite-transactions
