require 'sqlite3'

def mirrorlist_name_to_upstream_name(mirrorlist_name)
    if mirrorlist_name == 'mirrorlist'
        'arch'
    else
        mirrorlist_name.gsub('-mirrorlist', '').gsub('-', '')
    end
end

db = SQLite3::Database.new('dev.db')
known_upstreams = db.execute('select name from upstreams').map { |row| row[0] }

mirrorlist_glob = '/etc/pacman.d/*mirrorlist'
Dir.glob(mirrorlist_glob).each do |mirrorlist_file|
  upstream_name = mirrorlist_name_to_upstream_name(File.basename(mirrorlist_file))
  unless known_upstreams.include?(upstream_name)
    db.execute('insert into upstreams (name, upstream_type) values (?, ?)', [upstream_name, 'arch'])
  end
  upstream_id = db.execute('select id from upstreams where name = ?', upstream_name).map { |row| row[0] }
  known_mirrors = db.execute('select url from upstream_mirrors where upstream_id = ?', [upstream_id]).map { |row| row[0] }

  mirrors = File.readlines(mirrorlist_file).filter { |l| !l.include?('#') && l.include?('Server = ') }.map { |l| l.gsub('Server = ', '')}
  mirrors.each do |mir|
    unless known_mirrors.include?(mir)
        db.execute('insert into upstream_mirrors (upstream_id, url) values (?, ?)', [upstream_id, mir.chomp])
    end
  end
end

puts db.execute('select * from upstreams')
puts db.execute('select * from upstream_mirrors')