defmodule Bdb.Header do
  @spec parse(page :: binary()) :: %Bdb.Header.Metadata{} | %Bdb.Header.Btree{}
  def parse(<<
        lsn::unsigned-integer-little-size(64),
        pgno::unsigned-integer-little-size(32),
        0x00053162::unsigned-integer-little-size(32),
        version::unsigned-integer-little-size(32),
        pagesize::unsigned-integer-little-size(32),
        ec::unsigned-size(8),
        ty::unsigned-size(8),
        mf::unsigned-size(8),
        _::size(8),
        free::unsigned-integer-little-size(32),
        last_pgno::unsigned-integer-little-size(32),
        nparts::unsigned-integer-little-size(32),
        key_count::unsigned-integer-little-size(32),
        record_count::unsigned-integer-little-size(32),
        flags::unsigned-integer-little-size(32),
        uid::bytes-size(20),
        minkey::unsigned-integer-little-size(32),
        re_len::unsigned-integer-little-size(32),
        re_pad::unsigned-integer-little-size(32),
        root::unsigned-integer-little-size(32),
        _rest::binary
      >>) do
    %{
      lsn: lsn,
      pgno: pgno,
      version: version,
      pagesize: pagesize,
      ec: ec,
      ty: ty,
      mf: mf,
      free: free,
      nparts: nparts,
      key_count: key_count,
      last_pgno: last_pgno,
      record_count: record_count,
      flags: flags,
      uid: uid,
      minkey: minkey,
      re_len: re_len,
      re_pad: re_pad,
      root: root
    }
  end

  # def parse(page) do
  #   case page do
  #     <<lsn::unsigned-integer-little-size(64), pgno::unsigned-integer-little-size(32),
  #       0x00053162::unsigned-integer-little-size(32), version::unsigned-integer-little-size(32),
  #       pagesize::unsigned-integer-little-size(32), ec::unsigned-size(8), ty::unsigned-size(8),
  #       , _::size(8), free::unsigned-integer-little-size(32),
  #       last_pgno::unsigned-integer-little-size(32), nparts::unsigned-integer-little-size(32),
  #       key_count::unsigned-integer-little-size(32),
  #       record_count::unsigned-integer-little-size(32), flags::unsigned-integer-little-size(32),
  #       uid::bytes-size(20), minkey::unsigned-integer-little-size(32),
  #       re_len::unsigned-integer-little-size(32), re_pad::unsigned-integer-little-size(32),
  #       root::unsigned-integer-little-size(32), _::size(368),
  #       crypto_magic::unsigned-integer-little-size(32), iv::size(128), chksum::size(20),
  #       _::bytes>> ->
  #       %Bdb.Header.Metadata{
  #         lsn: lsn,
  #         pgno: pgno,
  #         version: version,
  #         pagesize: pagesize,
  #         ec: ec,
  #         ty: ty,
  #         mf: mf,
  #         free: free,
  #         last_pgno: last_pgno,
  #         nparts: nparts,
  #         key_count: key_count,
  #         record_count: record_count,
  #         flags: flags,
  #         uid: uid,
  #         minkey: minkey,
  #         re_len: re_len,
  #         re_pad: re_pad,
  #         root: root,
  #         crypto_magic: crypto_magic,
  #         iv: iv,
  #         chksum: chksum
  #       }
  #   end
  # end
end
