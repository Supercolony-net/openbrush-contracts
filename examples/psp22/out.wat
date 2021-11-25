(module
  (type (;0;) (func (param i32 i32 i32) (result i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param i32 i32 i32)))
  (type (;3;) (func (param i32 i32 i64 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;4;) (func (param i32 i32)))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (param i32 i32 i64 i64)))
  (type (;7;) (func (param i32) (result i32)))
  (type (;8;) (func))
  (type (;9;) (func (param i32 i32 i64)))
  (type (;10;) (func (param i32 i32 i32 i64 i64)))
  (type (;11;) (func (param i32 i32 i32 i64 i64 i32)))
  (type (;12;) (func (param i64 i64 i32)))
  (type (;13;) (func (param i32 i32 i32 i32)))
  (type (;14;) (func (param i32) (result i64)))
  (type (;15;) (func (result i32)))
  (type (;16;) (func (param i32 i32 i32 i32 i32)))
  (type (;17;) (func (param i32 i32 i32 i32 i32 i32 i32) (result i32)))
  (type (;18;) (func (param i32 i64) (result i32)))
  (import "env" "memory" (memory (;0;) 2))
  (import "seal0" "seal_set_storage" (func $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E (type 2)))
  (import "seal0" "seal_call" (func $_ZN7ink_env6engine8on_chain3ext3sys9seal_call17hcadb176a98e0eed8E (type 3)))
  (import "seal0" "seal_caller" (func $_ZN7ink_env6engine8on_chain3ext3sys11seal_caller17hf527ac368aa51f8aE (type 4)))
  (import "seal0" "seal_value_transferred" (func $_ZN7ink_env6engine8on_chain3ext3sys22seal_value_transferred17h916ed8215b2046a7E (type 4)))
  (import "seal0" "seal_clear_storage" (func $_ZN7ink_env6engine8on_chain3ext3sys18seal_clear_storage17ha0e0a8a3baefb6bcE (type 5)))
  (import "seal0" "seal_get_storage" (func $_ZN7ink_env6engine8on_chain3ext3sys16seal_get_storage17h26468309f0d4e343E (type 0)))
  (import "seal0" "seal_input" (func $_ZN7ink_env6engine8on_chain3ext3sys10seal_input17h7fb0017c5c621ea9E (type 4)))
  (import "seal0" "seal_return" (func $_ZN7ink_env6engine8on_chain3ext3sys11seal_return17h1344b67397975fd3E (type 2)))
  (import "seal0" "seal_hash_blake2_256" (func $_ZN7ink_env6engine8on_chain3ext3sys20seal_hash_blake2_25617h93084b8f47746e4fE (type 2)))
  (func $_ZN11ink_storage11collections7hashmap24HashMap$LT$K$C$V$C$H$GT$6insert17hd1bfc3a3e6a0334aE (type 6) (param i32 i32 i64 i64)
    (local i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 160
    i32.sub
    local.tee 4
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.const 112
        i32.add
        local.get 1
        call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$11lazily_load17hcfed61e18df0657dE
        local.tee 5
        i64.load
        i64.const 1
        i64.ne
        br_if 0 (;@2;)
        local.get 5
        local.get 2
        i64.store offset=8
        local.get 5
        i32.const 0
        i32.store8 offset=32
        local.get 5
        i32.const 16
        i32.add
        local.get 3
        i64.store
        br 1 (;@1;)
      end
      local.get 4
      i32.const 24
      i32.add
      local.tee 6
      local.get 1
      i32.const 24
      i32.add
      i64.load align=1
      i64.store
      local.get 4
      i32.const 16
      i32.add
      local.tee 7
      local.get 1
      i32.const 16
      i32.add
      i64.load align=1
      i64.store
      local.get 4
      i32.const 8
      i32.add
      local.tee 8
      local.get 1
      i32.const 8
      i32.add
      i64.load align=1
      i64.store
      local.get 4
      local.get 1
      i64.load align=1
      i64.store
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.const 44
                                i32.add
                                i32.load
                                local.tee 5
                                local.get 0
                                i32.const 48
                                i32.add
                                i32.load
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 0
                                i32.const 56
                                i32.add
                                local.tee 9
                                local.get 0
                                i32.load offset=40
                                local.tee 5
                                call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$11lazily_load17h92dffc5151c316a7E
                                local.set 10
                                local.get 4
                                i32.const 65
                                i32.add
                                local.get 8
                                i64.load
                                i64.store align=1
                                local.get 4
                                i32.const 73
                                i32.add
                                local.get 7
                                i64.load
                                i64.store align=1
                                local.get 4
                                i32.const 81
                                i32.add
                                local.get 6
                                i64.load
                                i64.store align=1
                                local.get 4
                                i32.const 1
                                i32.store8 offset=56
                                local.get 4
                                local.get 4
                                i64.load
                                i64.store offset=57 align=1
                                local.get 4
                                i32.const 112
                                i32.add
                                local.get 10
                                local.get 4
                                i32.const 56
                                i32.add
                                call $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$3put17h17a3991bdde32244E
                                local.get 4
                                i32.load8_u offset=112
                                local.tee 6
                                i32.const 2
                                i32.eq
                                br_if 8 (;@6;)
                                local.get 6
                                i32.const 1
                                i32.eq
                                br_if 11 (;@3;)
                                local.get 4
                                i32.load offset=116
                                local.set 6
                                block  ;; label = @15
                                  local.get 4
                                  i32.const 120
                                  i32.add
                                  i32.load
                                  local.tee 7
                                  local.get 5
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 6
                                  local.get 5
                                  i32.eq
                                  br_if 4 (;@11;)
                                end
                                local.get 9
                                local.get 7
                                call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$7get_mut17h1c4d658fd7bc8d8dE
                                local.tee 8
                                br_if 1 (;@13;)
                                i32.const 0
                                local.set 8
                                br 2 (;@12;)
                              end
                              local.get 4
                              i32.const 32
                              i32.add
                              local.get 0
                              i32.const 96
                              i32.add
                              local.get 5
                              call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17heda31ca3082bdc94E
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 4
                                  i32.load offset=32
                                  i32.const 1
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 4
                                  i32.const 56
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.get 4
                                  i32.const 52
                                  i32.add
                                  i32.load
                                  i32.store
                                  local.get 4
                                  i32.const 56
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.get 4
                                  i32.const 44
                                  i32.add
                                  i64.load align=4
                                  i64.store
                                  local.get 4
                                  local.get 4
                                  i64.load offset=36 align=4
                                  i64.store offset=56
                                  i32.const 40
                                  i32.const 4
                                  call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E
                                  local.tee 6
                                  i32.const 1
                                  i32.store8
                                  local.get 6
                                  i32.const 0
                                  i32.store8 offset=36
                                  local.get 6
                                  local.get 4
                                  i64.load
                                  i64.store offset=1 align=1
                                  local.get 6
                                  i32.const 9
                                  i32.add
                                  local.get 4
                                  i32.const 8
                                  i32.add
                                  i64.load
                                  i64.store align=1
                                  local.get 6
                                  i32.const 17
                                  i32.add
                                  local.get 4
                                  i32.const 16
                                  i32.add
                                  i64.load
                                  i64.store align=1
                                  local.get 6
                                  i32.const 25
                                  i32.add
                                  local.get 4
                                  i32.const 24
                                  i32.add
                                  i64.load
                                  i64.store align=1
                                  local.get 4
                                  i32.const 56
                                  i32.add
                                  local.get 6
                                  call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17had0422e91c1adb57E
                                  drop
                                  br 1 (;@14;)
                                end
                                local.get 4
                                i32.const 32
                                i32.add
                                i32.const 8
                                i32.add
                                i32.load
                                local.get 4
                                i32.const 44
                                i32.add
                                i32.load
                                i32.const 2
                                i32.shl
                                i32.add
                                i32.const 48
                                i32.add
                                i32.load
                                local.set 6
                                local.get 4
                                i32.const 65
                                i32.add
                                local.get 4
                                i32.const 8
                                i32.add
                                i64.load
                                i64.store align=1
                                local.get 4
                                i32.const 73
                                i32.add
                                local.get 4
                                i32.const 16
                                i32.add
                                i64.load
                                i64.store align=1
                                local.get 4
                                i32.const 81
                                i32.add
                                local.get 4
                                i32.const 24
                                i32.add
                                i64.load
                                i64.store align=1
                                local.get 4
                                i32.const 1
                                i32.store8 offset=56
                                local.get 4
                                local.get 4
                                i64.load
                                i64.store offset=57 align=1
                                local.get 4
                                i32.const 112
                                i32.add
                                local.get 6
                                local.get 4
                                i32.const 56
                                i32.add
                                call $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$3put17h17a3991bdde32244E
                              end
                              local.get 0
                              i32.load offset=40
                              local.tee 6
                              i32.const 1
                              i32.add
                              local.tee 7
                              local.get 6
                              i32.lt_u
                              br_if 10 (;@3;)
                              local.get 0
                              local.get 7
                              i32.store offset=40
                              local.get 0
                              i32.load offset=48
                              local.tee 6
                              i32.const 1
                              i32.add
                              local.tee 7
                              local.get 6
                              i32.lt_u
                              br_if 10 (;@3;)
                              local.get 0
                              local.get 7
                              i32.store offset=48
                              br 9 (;@4;)
                            end
                            i32.const 0
                            local.get 8
                            i32.const 4
                            i32.add
                            local.get 8
                            i32.load8_u
                            i32.const 1
                            i32.eq
                            select
                            local.set 8
                          end
                          local.get 8
                          call $_ZN4core6option15Option$LT$T$GT$6expect17hbfd96ec228071fcaE
                          local.set 8
                          local.get 7
                          local.get 6
                          i32.eq
                          br_if 3 (;@8;)
                          local.get 8
                          local.get 6
                          i32.store
                          local.get 9
                          local.get 6
                          call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$7get_mut17h1c4d658fd7bc8d8dE
                          local.tee 8
                          br_if 1 (;@10;)
                          i32.const 0
                          local.set 8
                          br 2 (;@9;)
                        end
                        local.get 0
                        i32.load offset=44
                        local.set 6
                        br 5 (;@5;)
                      end
                      i32.const 0
                      local.get 8
                      i32.const 4
                      i32.add
                      local.get 8
                      i32.load8_u
                      i32.const 1
                      i32.eq
                      select
                      local.set 8
                    end
                    local.get 8
                    call $_ZN4core6option15Option$LT$T$GT$6expect17hbfd96ec228071fcaE
                    i32.const 4
                    i32.add
                    local.set 8
                    br 1 (;@7;)
                  end
                  local.get 8
                  local.get 6
                  i32.store offset=4
                end
                local.get 8
                local.get 7
                i32.store
                local.get 0
                i32.load offset=40
                local.get 5
                i32.ne
                br_if 2 (;@4;)
                local.get 6
                local.get 7
                local.get 7
                local.get 6
                i32.gt_u
                select
                local.set 6
                br 1 (;@5;)
              end
              call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
              unreachable
            end
            local.get 0
            local.get 6
            i32.store offset=40
          end
          local.get 0
          i32.load offset=44
          local.tee 6
          i32.const 1
          i32.add
          local.tee 7
          local.get 6
          i32.ge_u
          br_if 1 (;@2;)
        end
        unreachable
        unreachable
      end
      local.get 0
      local.get 7
      i32.store offset=44
      local.get 4
      i32.const 24
      i32.add
      local.tee 6
      local.get 1
      i32.const 24
      i32.add
      i64.load align=1
      i64.store
      local.get 4
      i32.const 16
      i32.add
      local.tee 7
      local.get 1
      i32.const 16
      i32.add
      i64.load align=1
      i64.store
      local.get 4
      i32.const 8
      i32.add
      local.tee 8
      local.get 1
      i32.const 8
      i32.add
      i64.load align=1
      i64.store
      local.get 4
      local.get 1
      i64.load align=1
      i64.store
      i32.const 40
      i32.const 8
      call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E
      local.tee 1
      local.get 2
      i64.store offset=8
      local.get 1
      i32.const 0
      i32.store8 offset=32
      local.get 1
      local.get 5
      i32.store offset=24
      local.get 1
      i64.const 1
      i64.store
      local.get 1
      i32.const 16
      i32.add
      local.get 3
      i64.store
      local.get 4
      i32.const 112
      i32.add
      i32.const 24
      i32.add
      local.get 6
      i64.load
      i64.store
      local.get 4
      i32.const 112
      i32.add
      i32.const 16
      i32.add
      local.get 7
      i64.load
      i64.store
      local.get 4
      i32.const 112
      i32.add
      i32.const 8
      i32.add
      local.get 8
      i64.load
      i64.store
      local.get 4
      local.get 4
      i64.load
      i64.store offset=112
      local.get 4
      i32.const 56
      i32.add
      local.get 0
      i32.const 152
      i32.add
      local.get 4
      i32.const 112
      i32.add
      call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17hb33a4c84e26b393aE
      block  ;; label = @2
        local.get 4
        i32.load offset=56
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 4
        i32.const 112
        i32.add
        local.get 4
        i32.const 56
        i32.add
        i32.const 4
        i32.or
        i32.const 48
        call $memcpy
        drop
        local.get 4
        i32.const 112
        i32.add
        local.get 1
        call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17hb3a2e491d96ed1e5E
        drop
        br 1 (;@1;)
      end
      local.get 4
      i32.const 56
      i32.add
      i32.const 8
      i32.add
      i32.load
      local.get 4
      i32.const 68
      i32.add
      i32.load
      i32.const 2
      i32.shl
      i32.add
      i32.const 4
      i32.add
      local.get 1
      i32.store
    end
    local.get 4
    i32.const 160
    i32.add
    global.set 0)
  (func $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$11lazily_load17hcfed61e18df0657dE (type 1) (param i32 i32) (result i32)
    (local i32 i64 i32 i32 i32 i64)
    global.get 0
    i32.const 176
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 96
    i32.add
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get 2
    i32.const 96
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get 2
    i32.const 96
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get 2
    local.get 1
    i64.load align=1
    i64.store offset=96
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.const 40
    i32.add
    local.get 2
    i32.const 96
    i32.add
    call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17hb33a4c84e26b393aE
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        i64.const 2
        local.set 3
        block  ;; label = @3
          local.get 0
          i64.load
          i64.const 1
          i64.ne
          br_if 0 (;@3;)
          local.get 2
          i32.const 144
          i32.add
          i32.const 24
          i32.add
          local.tee 4
          local.get 0
          i32.const 32
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 144
          i32.add
          i32.const 16
          i32.add
          local.tee 5
          local.get 0
          i32.const 24
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 144
          i32.add
          i32.const 8
          i32.add
          local.tee 6
          local.get 0
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 0
          i64.load offset=8
          i64.store offset=144
          local.get 2
          i32.const 96
          i32.add
          i32.const 8
          i32.add
          local.get 2
          i32.const 144
          i32.add
          local.get 1
          call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$13to_offset_key17h913ecdf8713f958cE
          local.get 4
          local.get 2
          i32.const 96
          i32.add
          i32.const 32
          i32.add
          i64.load
          i64.store
          local.get 5
          local.get 2
          i32.const 96
          i32.add
          i32.const 24
          i32.add
          i64.load
          i64.store
          local.get 6
          local.get 2
          i32.const 96
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 2
          i64.load offset=104
          i64.store offset=144
          local.get 2
          i32.const 64
          i32.add
          local.get 2
          i32.const 144
          i32.add
          call $_ZN11ink_storage6traits7optspec20pull_packed_root_opt17ha3f29b1c536f4bc0E
          local.get 2
          i64.load offset=64
          local.set 3
        end
        local.get 2
        i32.const 8
        i32.add
        i32.const 4
        i32.or
        local.set 1
        i64.const 0
        local.set 7
        block  ;; label = @3
          local.get 3
          i64.const 2
          i64.eq
          br_if 0 (;@3;)
          local.get 2
          i32.const 144
          i32.add
          i32.const 16
          i32.add
          local.get 2
          i32.const 88
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 152
          i32.add
          local.get 2
          i32.const 64
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 2
          i64.load offset=72
          i64.store offset=144
          local.get 3
          local.set 7
        end
        local.get 2
        i32.const 96
        i32.add
        local.get 1
        i32.const 48
        call $memcpy
        drop
        i32.const 40
        i32.const 8
        call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E.240
        local.tee 1
        local.get 7
        i64.store
        local.get 1
        i32.const 1
        i32.store8 offset=32
        local.get 1
        local.get 2
        i64.load offset=144
        i64.store offset=8
        local.get 1
        i32.const 16
        i32.add
        local.get 2
        i32.const 144
        i32.add
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 1
        i32.const 24
        i32.add
        local.get 2
        i32.const 144
        i32.add
        i32.const 16
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 96
        i32.add
        local.get 1
        call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17hb3a2e491d96ed1e5E
        i32.load
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      i32.add
      i32.load
      local.get 2
      i32.const 20
      i32.add
      i32.load
      i32.const 2
      i32.shl
      i32.add
      i32.const 4
      i32.add
      i32.load
      local.set 1
    end
    local.get 2
    i32.const 176
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$11lazily_load17h92dffc5151c316a7E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i64 i64 i64 i64 i64 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 176
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.const 40
    i32.add
    local.get 1
    call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17heda31ca3082bdc94E
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=8
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          i32.const 2
          local.set 3
          block  ;; label = @4
            local.get 0
            i64.load
            i64.const 1
            i64.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 32
            i32.add
            i64.load
            local.set 4
            local.get 0
            i32.const 24
            i32.add
            i64.load
            local.set 5
            local.get 0
            i32.const 16
            i32.add
            i64.load
            local.set 6
            local.get 2
            local.get 0
            i64.load offset=8
            local.tee 7
            local.get 1
            i64.extend_i32_u
            i64.add
            local.tee 8
            i64.store offset=56
            local.get 2
            local.get 6
            local.get 8
            local.get 7
            i64.lt_u
            i64.extend_i32_u
            i64.add
            local.tee 7
            i64.store offset=64
            local.get 2
            local.get 5
            local.get 7
            local.get 6
            i64.lt_u
            i64.extend_i32_u
            i64.add
            local.tee 6
            i64.store offset=72
            local.get 2
            local.get 4
            local.get 6
            local.get 5
            i64.lt_u
            i64.extend_i32_u
            i64.add
            i64.store offset=80
            local.get 2
            i32.const 16384
            i32.store offset=100
            local.get 2
            i32.const 68528
            i32.store offset=96
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 56
                i32.add
                local.get 2
                i32.const 96
                i32.add
                call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
                local.tee 0
                i32.const 3
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                i32.const 13
                i32.ne
                br_if 5 (;@1;)
                local.get 2
                local.get 2
                i64.load offset=96
                i64.store offset=32
                local.get 2
                local.get 2
                i32.const 32
                i32.add
                call $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E
                block  ;; label = @7
                  local.get 2
                  i32.load8_u
                  i32.const 1
                  i32.and
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.load8_u offset=1
                        i32.const 255
                        i32.and
                        br_table 0 (;@10;) 1 (;@9;) 3 (;@7;)
                      end
                      local.get 2
                      i32.const 136
                      i32.add
                      local.get 2
                      i32.const 32
                      i32.add
                      call $_ZN11ink_storage11collections5stash1_108_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_storage..collections..stash..VacantEntry$GT$6decode17heeaed37a09162c5eE
                      local.get 2
                      i32.load offset=136
                      i32.const 1
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 2
                      i32.const 144
                      i32.add
                      i32.load
                      local.set 9
                      local.get 2
                      i32.load offset=140
                      local.set 10
                      i32.const 0
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 2
                    i32.const 136
                    i32.add
                    local.get 2
                    i32.const 32
                    i32.add
                    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                    i32.const 1
                    local.set 3
                    local.get 2
                    i32.load8_u offset=136
                    i32.const 1
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 2
                    i32.const 134
                    i32.add
                    local.get 2
                    i32.load8_u offset=139
                    i32.store8
                    local.get 2
                    i32.const 104
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 2
                    i32.const 156
                    i32.add
                    i64.load align=4
                    i64.store
                    local.get 2
                    i32.const 117
                    i32.add
                    local.get 2
                    i32.const 161
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 2
                    local.get 2
                    i32.load16_u offset=137 align=1
                    i32.store16 offset=132
                    local.get 2
                    local.get 2
                    i32.const 148
                    i32.add
                    i64.load align=4
                    i64.store offset=104
                    local.get 2
                    i32.const 136
                    i32.add
                    i32.const 8
                    i32.add
                    i32.load
                    local.set 9
                    local.get 2
                    i32.load offset=140
                    local.set 10
                  end
                  local.get 2
                  i32.const 92
                  i32.add
                  i32.const 2
                  i32.add
                  local.get 2
                  i32.const 132
                  i32.add
                  i32.const 2
                  i32.add
                  i32.load8_u
                  i32.store8
                  local.get 2
                  i32.const 136
                  i32.add
                  i32.const 8
                  i32.add
                  local.get 2
                  i32.const 104
                  i32.add
                  i32.const 8
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 136
                  i32.add
                  i32.const 16
                  i32.add
                  local.get 2
                  i32.const 104
                  i32.add
                  i32.const 16
                  i32.add
                  i64.load
                  i64.store
                  local.get 2
                  local.get 2
                  i32.load16_u offset=132
                  i32.store16 offset=92
                  local.get 2
                  local.get 2
                  i64.load offset=104
                  i64.store offset=136
                  br 2 (;@5;)
                end
                call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
                unreachable
              end
              i32.const 2
              local.set 3
            end
            local.get 2
            i32.const 96
            i32.add
            i32.const 2
            i32.add
            local.tee 11
            local.get 2
            i32.const 92
            i32.add
            i32.const 2
            i32.add
            local.tee 12
            i32.load8_u
            i32.store8
            local.get 2
            i32.const 104
            i32.add
            i32.const 8
            i32.add
            local.tee 13
            local.get 2
            i32.const 136
            i32.add
            i32.const 8
            i32.add
            local.tee 0
            i64.load
            i64.store
            local.get 2
            i32.const 104
            i32.add
            i32.const 16
            i32.add
            local.tee 14
            local.get 2
            i32.const 136
            i32.add
            i32.const 16
            i32.add
            local.tee 1
            i64.load
            i64.store
            local.get 2
            local.get 2
            i32.load16_u offset=92
            i32.store16 offset=96
            local.get 2
            local.get 2
            i64.load offset=136
            i64.store offset=104
            block  ;; label = @5
              local.get 3
              i32.const 2
              i32.eq
              br_if 0 (;@5;)
              local.get 2
              i32.const 132
              i32.add
              i32.const 2
              i32.add
              local.get 11
              i32.load8_u
              i32.store8
              local.get 0
              local.get 13
              i64.load
              i64.store
              local.get 1
              local.get 14
              i64.load
              i64.store
              local.get 2
              local.get 2
              i32.load16_u offset=96
              i32.store16 offset=132
              local.get 2
              local.get 2
              i64.load offset=104
              i64.store offset=136
            end
            local.get 12
            local.get 2
            i32.const 132
            i32.add
            i32.const 2
            i32.add
            i32.load8_u
            i32.store8
            local.get 2
            i32.const 32
            i32.add
            i32.const 8
            i32.add
            local.get 0
            i64.load
            i64.store
            local.get 2
            i32.const 32
            i32.add
            i32.const 16
            i32.add
            local.get 1
            i64.load
            i64.store
            local.get 2
            local.get 2
            i32.load16_u offset=132
            i32.store16 offset=92
            local.get 2
            local.get 2
            i64.load offset=136
            i64.store offset=32
          end
          local.get 2
          i32.const 136
          i32.add
          i32.const 16
          i32.add
          local.get 2
          i32.const 8
          i32.add
          i32.const 4
          i32.or
          local.tee 0
          i32.const 16
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 136
          i32.add
          i32.const 8
          i32.add
          local.get 0
          i32.const 8
          i32.add
          i64.load align=4
          i64.store
          local.get 2
          local.get 0
          i64.load align=4
          i64.store offset=136
          i32.const 40
          i32.const 4
          call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E.240
          local.tee 0
          local.get 3
          i32.store8
          local.get 0
          local.get 9
          i32.store offset=8 align=1
          local.get 0
          local.get 10
          i32.store offset=4 align=1
          local.get 0
          i32.const 1
          i32.store8 offset=36
          local.get 0
          local.get 2
          i32.load16_u offset=92
          i32.store16 offset=1 align=1
          local.get 0
          i32.const 3
          i32.add
          local.get 2
          i32.const 94
          i32.add
          i32.load8_u
          i32.store8
          local.get 0
          local.get 2
          i64.load offset=32
          i64.store offset=12 align=1
          local.get 0
          i32.const 20
          i32.add
          local.get 2
          i32.const 32
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store align=1
          local.get 0
          i32.const 28
          i32.add
          local.get 2
          i32.const 32
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store align=1
          local.get 2
          i32.const 136
          i32.add
          local.get 0
          call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17had0422e91c1adb57E
          i32.load
          local.set 0
          br 1 (;@2;)
        end
        local.get 2
        i32.const 16
        i32.add
        i32.load
        local.get 2
        i32.const 20
        i32.add
        i32.load
        i32.const 2
        i32.shl
        i32.add
        i32.const 48
        i32.add
        i32.load
        local.set 0
      end
      local.get 2
      i32.const 176
      i32.add
      global.set 0
      local.get 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$3put17h17a3991bdde32244E (type 2) (param i32 i32 i32)
    (local i32)
    local.get 2
    i32.load8_u
    local.set 3
    local.get 0
    local.get 1
    i32.const 36
    call $memcpy
    local.set 0
    local.get 1
    local.get 2
    i32.const 36
    call $memcpy
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 2
        i32.ne
        br_if 0 (;@2;)
        local.get 3
        i32.const 255
        i32.and
        i32.const 2
        i32.eq
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const 0
      i32.store8 offset=36
    end)
  (func $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$7get_mut17h1c4d658fd7bc8d8dE (type 1) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    local.set 2
    block  ;; label = @1
      local.get 0
      local.get 1
      call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$11lazily_load17h92dffc5151c316a7E
      local.tee 0
      i32.load8_u
      i32.const 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store8 offset=36
      local.get 0
      local.set 2
    end
    local.get 2)
  (func $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17heda31ca3082bdc94E (type 2) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17hbbff3628aecd05a3E
      local.tee 3
      i32.store offset=4
      i32.const 0
      local.set 4
      local.get 1
      i32.const 0
      i32.store
    end
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.load16_u offset=94
        local.tee 5
        i32.const 2
        i32.shl
        local.set 6
        i32.const 0
        local.set 7
        i32.const -1
        local.set 8
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 6
              local.get 7
              i32.ne
              br_if 0 (;@5;)
              local.get 5
              local.set 8
              br 2 (;@3;)
            end
            local.get 3
            local.get 7
            i32.add
            local.set 9
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 7
            i32.const 4
            i32.add
            local.set 7
            block  ;; label = @5
              i32.const -1
              local.get 9
              i32.const 4
              i32.add
              i32.load
              local.tee 9
              local.get 2
              i32.ne
              local.get 9
              local.get 2
              i32.gt_u
              select
              i32.const 1
              i32.add
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
          end
          local.get 0
          i32.const 12
          i32.add
          local.get 8
          i32.store
          local.get 0
          i32.const 8
          i32.add
          local.get 3
          i32.store
          local.get 0
          i32.const 16
          i32.add
          local.set 7
          i32.const 1
          local.set 8
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const -1
          i32.add
          local.set 4
          local.get 3
          local.get 8
          i32.const 2
          i32.shl
          i32.add
          i32.const 96
          i32.add
          i32.load
          local.set 3
          br 1 (;@2;)
        end
      end
      local.get 0
      i32.const 16
      i32.add
      local.get 8
      i32.store
      local.get 0
      i32.const 12
      i32.add
      local.get 3
      i32.store
      i32.const 0
      local.set 8
      local.get 0
      i32.const 8
      i32.add
      i32.const 0
      i32.store
      local.get 0
      i32.const 20
      i32.add
      local.set 7
      local.get 2
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 7
    local.get 1
    i32.store
    local.get 0
    local.get 8
    i32.store)
  (func $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E (type 1) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      local.get 1
      call $_ZN87_$LT$ink_allocator..bump..BumpAllocator$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h3b5b87aeed817ef7E
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0)
  (func $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17had0422e91c1adb57E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 3
    local.get 2
    i32.const 48
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 12
    i32.add
    i32.load
    i32.store
    local.get 2
    local.get 0
    i64.load offset=4 align=4
    i64.store offset=48
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.load offset=52
                  local.tee 4
                  i32.load16_u offset=94
                  i32.const 11
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 2
                  i32.load offset=56
                  call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i32.load
                  local.set 5
                  local.get 2
                  i32.load offset=68
                  local.set 6
                  local.get 2
                  i32.load offset=64
                  local.set 7
                  local.get 2
                  i32.load offset=48
                  local.set 8
                  call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17hbbff3628aecd05a3E
                  local.set 9
                  local.get 4
                  i32.load16_u offset=94
                  local.tee 10
                  local.get 7
                  i32.sub
                  local.tee 11
                  local.get 10
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 11
                  i32.const -1
                  i32.add
                  local.tee 12
                  local.get 11
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 9
                  local.get 12
                  i32.store16 offset=94
                  local.get 7
                  i32.const 1
                  i32.add
                  local.tee 13
                  local.get 7
                  i32.lt_u
                  br_if 6 (;@1;)
                  local.get 10
                  local.get 13
                  i32.sub
                  local.tee 11
                  local.get 10
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 7
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 10
                  i32.const 4
                  i32.add
                  i32.load
                  local.set 14
                  local.get 10
                  i32.const 48
                  i32.add
                  i32.load
                  local.set 15
                  local.get 2
                  i32.const 40
                  i32.add
                  local.get 9
                  i32.const 4
                  i32.add
                  local.get 12
                  call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h60030e9adf15a33cE
                  local.get 11
                  local.get 2
                  i32.load offset=44
                  i32.ne
                  br_if 6 (;@1;)
                  local.get 2
                  i32.load offset=40
                  local.get 4
                  local.get 13
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 10
                  i32.const 4
                  i32.add
                  local.get 11
                  i32.const 2
                  i32.shl
                  local.tee 13
                  call $memcpy
                  drop
                  local.get 2
                  i32.const 32
                  i32.add
                  local.get 9
                  i32.const 48
                  i32.add
                  local.get 12
                  call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h40a2b90a4374113bE
                  local.get 11
                  local.get 2
                  i32.load offset=36
                  i32.ne
                  br_if 6 (;@1;)
                  local.get 2
                  i32.load offset=32
                  local.get 10
                  i32.const 48
                  i32.add
                  local.get 13
                  call $memcpy
                  drop
                  local.get 4
                  local.get 7
                  i32.store16 offset=94
                  local.get 2
                  local.get 5
                  i32.store offset=72
                  local.get 2
                  local.get 9
                  local.get 4
                  local.get 6
                  select
                  i32.store offset=68
                  i32.const 0
                  local.set 10
                  local.get 2
                  i32.const 0
                  local.get 8
                  local.get 6
                  select
                  i32.store offset=64
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 3
                  local.get 1
                  call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17he9bc6c8782f9e8ceE
                  local.set 16
                  loop  ;; label = @8
                    local.get 4
                    i32.load
                    local.tee 7
                    i32.eqz
                    br_if 2 (;@6;)
                    local.get 8
                    i32.const 1
                    i32.add
                    local.tee 1
                    local.get 8
                    i32.lt_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 4
                    i32.load16_u offset=92
                    local.tee 8
                    i32.store offset=56
                    local.get 2
                    local.get 7
                    i32.store offset=52
                    local.get 2
                    local.get 1
                    i32.store offset=48
                    local.get 1
                    i32.const -1
                    i32.add
                    local.tee 4
                    local.get 1
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 4
                    local.get 10
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 7
                    i32.load16_u offset=94
                    i32.const 11
                    i32.lt_u
                    br_if 4 (;@4;)
                    local.get 2
                    i32.const 64
                    i32.add
                    local.get 8
                    call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
                    local.get 2
                    i32.load offset=72
                    local.set 13
                    local.get 2
                    i32.load offset=68
                    local.set 5
                    local.get 2
                    i32.load offset=64
                    local.set 8
                    local.get 7
                    i32.load16_u offset=94
                    local.set 6
                    call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h9c6c530ccfa8ad4cE
                    local.set 4
                    local.get 7
                    i32.load16_u offset=94
                    local.tee 10
                    local.get 8
                    i32.sub
                    local.tee 12
                    local.get 10
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 12
                    i32.const -1
                    i32.add
                    local.tee 3
                    local.get 12
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 4
                    local.get 3
                    i32.store16 offset=94
                    local.get 8
                    i32.const 1
                    i32.add
                    local.tee 12
                    local.get 8
                    i32.lt_u
                    br_if 7 (;@1;)
                    local.get 10
                    local.get 12
                    i32.sub
                    local.tee 11
                    local.get 10
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 7
                    local.get 8
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 10
                    i32.const 4
                    i32.add
                    i32.load
                    local.set 17
                    local.get 10
                    i32.const 48
                    i32.add
                    i32.load
                    local.set 18
                    local.get 2
                    i32.const 24
                    i32.add
                    local.get 4
                    i32.const 4
                    i32.add
                    local.get 3
                    call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h60030e9adf15a33cE
                    local.get 11
                    local.get 2
                    i32.load offset=28
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 2
                    i32.load offset=24
                    local.get 7
                    local.get 12
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 10
                    i32.const 4
                    i32.add
                    local.get 11
                    i32.const 2
                    i32.shl
                    local.tee 19
                    call $memcpy
                    drop
                    local.get 2
                    i32.const 16
                    i32.add
                    local.get 4
                    i32.const 48
                    i32.add
                    local.get 3
                    call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h40a2b90a4374113bE
                    local.get 11
                    local.get 2
                    i32.load offset=20
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 2
                    i32.load offset=16
                    local.get 10
                    i32.const 48
                    i32.add
                    local.get 19
                    call $memcpy
                    drop
                    local.get 7
                    local.get 8
                    i32.store16 offset=94
                    local.get 6
                    i32.const 1
                    i32.add
                    local.tee 3
                    local.get 12
                    i32.sub
                    local.tee 8
                    local.get 3
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 4
                    i32.load16_u offset=94
                    local.tee 3
                    i32.const 12
                    i32.ge_u
                    br_if 3 (;@5;)
                    local.get 8
                    local.get 3
                    i32.const 1
                    i32.add
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 4
                    i32.const 96
                    i32.add
                    local.get 10
                    i32.const 96
                    i32.add
                    local.get 8
                    i32.const 2
                    i32.shl
                    call $memcpy
                    drop
                    local.get 2
                    i32.const 8
                    i32.add
                    local.get 4
                    local.get 1
                    call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17hbfba168d5d1170a0E
                    local.get 2
                    i32.load offset=12
                    local.set 4
                    local.get 2
                    i32.load offset=8
                    local.set 10
                    local.get 1
                    local.set 8
                    local.get 7
                    local.set 3
                    block  ;; label = @9
                      local.get 5
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 10
                      local.set 8
                      local.get 4
                      local.set 3
                    end
                    local.get 2
                    local.get 13
                    i32.store offset=72
                    local.get 2
                    local.get 3
                    i32.store offset=68
                    local.get 2
                    local.get 8
                    i32.store offset=64
                    local.get 2
                    i32.const 64
                    i32.add
                    local.get 14
                    local.get 15
                    local.get 9
                    call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17h45eb42203db36e8eE
                    local.get 1
                    local.set 8
                    local.get 4
                    local.set 9
                    local.get 18
                    local.set 15
                    local.get 17
                    local.set 14
                    local.get 7
                    local.set 4
                    br 0 (;@8;)
                  end
                end
                local.get 2
                i32.const 48
                i32.add
                local.get 3
                local.get 1
                call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17he9bc6c8782f9e8ceE
                local.set 16
                br 3 (;@3;)
              end
              local.get 0
              i32.load offset=16
              local.tee 1
              i32.load offset=4
              local.tee 4
              i32.eqz
              br_if 4 (;@1;)
              local.get 1
              i32.load
              local.set 7
              call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h9c6c530ccfa8ad4cE
              local.tee 8
              local.get 4
              i32.store offset=96
              local.get 7
              i32.const 1
              i32.add
              local.tee 4
              local.get 7
              i32.lt_u
              br_if 4 (;@1;)
              local.get 2
              local.get 8
              local.get 4
              call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17hbfba168d5d1170a0E
              local.get 2
              i32.load
              local.set 4
              local.get 1
              local.get 2
              i32.load offset=4
              local.tee 7
              i32.store offset=4
              local.get 1
              local.get 4
              i32.store
              local.get 4
              i32.const -1
              i32.add
              local.tee 8
              local.get 4
              i32.gt_u
              br_if 4 (;@1;)
              local.get 8
              local.get 10
              i32.ne
              br_if 4 (;@1;)
              local.get 7
              i32.load16_u offset=94
              local.tee 8
              i32.const 10
              i32.gt_u
              br_if 4 (;@1;)
              local.get 7
              local.get 8
              i32.const 1
              i32.add
              local.tee 10
              i32.store16 offset=94
              local.get 7
              local.get 8
              i32.const 2
              i32.shl
              i32.add
              local.tee 8
              i32.const 48
              i32.add
              local.get 15
              i32.store
              local.get 8
              i32.const 4
              i32.add
              local.get 14
              i32.store
              local.get 7
              local.get 10
              i32.const 2
              i32.shl
              i32.add
              i32.const 96
              i32.add
              local.get 9
              i32.store
              local.get 2
              local.get 10
              i32.store offset=72
              local.get 2
              local.get 7
              i32.store offset=68
              local.get 2
              local.get 4
              i32.store offset=64
              local.get 2
              i32.const 64
              i32.add
              call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17he4d991720d4aa946E
              local.get 1
              i32.load offset=8
              local.tee 4
              i32.const 1
              i32.add
              local.tee 7
              local.get 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              i32.const 8
              i32.add
              local.set 1
              br 3 (;@2;)
            end
            call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
            unreachable
          end
          local.get 2
          i32.const 48
          i32.add
          local.get 14
          local.get 15
          local.get 9
          call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17h45eb42203db36e8eE
        end
        local.get 0
        i32.load offset=16
        local.tee 4
        i32.load offset=8
        local.tee 1
        i32.const 1
        i32.add
        local.tee 7
        local.get 1
        i32.lt_u
        br_if 1 (;@1;)
        local.get 4
        i32.const 8
        i32.add
        local.set 1
      end
      local.get 1
      local.get 7
      i32.store
      local.get 2
      i32.const 80
      i32.add
      global.set 0
      local.get 16
      return
    end
    unreachable
    unreachable)
  (func $_ZN4core6option15Option$LT$T$GT$6expect17hbfd96ec228071fcaE (type 7) (param i32) (result i32)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
      unreachable
    end
    local.get 0)
  (func $_ZN4core6option13expect_failed17h076ee9a0697574d1E (type 8)
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17hb33a4c84e26b393aE (type 2) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17hf25fb16daee1c84eE
      local.tee 3
      i32.store offset=4
      i32.const 0
      local.set 4
      local.get 1
      i32.const 0
      i32.store
    end
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.load16_u offset=50
        local.tee 5
        i32.const 5
        i32.shl
        local.set 6
        i32.const 0
        local.set 7
        i32.const -1
        local.set 8
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 6
              local.get 7
              i32.ne
              br_if 0 (;@5;)
              local.get 5
              local.set 8
              br 2 (;@3;)
            end
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 3
            local.get 7
            i32.add
            local.set 9
            local.get 7
            i32.const 32
            i32.add
            local.set 7
            block  ;; label = @5
              local.get 2
              local.get 9
              i32.const 52
              i32.add
              call $_ZN60_$LT$ink_env..types..AccountId$u20$as$u20$core..cmp..Ord$GT$3cmp17h3e35a495ccaba03eE
              i32.const 24
              i32.shl
              i32.const 24
              i32.shr_s
              i32.const 1
              i32.add
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
          end
          i32.const 1
          local.set 7
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const -1
          i32.add
          local.set 4
          local.get 3
          local.get 8
          i32.const 2
          i32.shl
          i32.add
          i32.const 404
          i32.add
          i32.load
          local.set 3
          br 1 (;@2;)
        end
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 2
      i64.load align=1
      i64.store align=1
      local.get 0
      i32.const 44
      i32.add
      local.get 2
      i32.const 24
      i32.add
      i64.load align=1
      i64.store align=1
      local.get 0
      i32.const 36
      i32.add
      local.get 2
      i32.const 16
      i32.add
      i64.load align=1
      i64.store align=1
      local.get 0
      i32.const 28
      i32.add
      local.get 2
      i32.const 8
      i32.add
      i64.load align=1
      i64.store align=1
      i32.const 0
      local.set 4
      i32.const 0
      local.set 7
    end
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 0
    local.get 7
    i32.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 12
    i32.add
    local.get 8
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 3
    i32.store)
  (func $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17hb3a2e491d96ed1e5E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 240
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 48
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 2
    local.get 0
    i64.load align=4
    i64.store offset=48
    local.get 2
    i32.const 112
    i32.add
    i32.const 24
    i32.add
    local.get 0
    i32.const 40
    i32.add
    i64.load align=1
    i64.store
    local.get 2
    i32.const 112
    i32.add
    i32.const 16
    i32.add
    local.get 0
    i32.const 32
    i32.add
    i64.load align=1
    i64.store
    local.get 2
    i32.const 112
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get 2
    local.get 0
    i64.load offset=16 align=1
    i64.store offset=112
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=52
            local.tee 3
            i32.load16_u offset=50
            local.tee 4
            i32.const 11
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            i32.const 208
            i32.add
            local.get 2
            i32.load offset=56
            call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
            local.get 2
            i32.const 208
            i32.add
            i32.const 8
            i32.add
            i32.load
            local.set 5
            local.get 2
            i32.load offset=212
            local.set 6
            local.get 2
            i32.load offset=208
            local.set 7
            local.get 2
            i32.load offset=48
            local.set 8
            call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17hf25fb16daee1c84eE
            local.set 9
            local.get 3
            i32.load16_u offset=50
            local.tee 10
            local.get 7
            i32.sub
            local.tee 11
            local.get 10
            i32.gt_u
            br_if 2 (;@2;)
            local.get 11
            i32.const -1
            i32.add
            local.tee 12
            local.get 11
            i32.gt_u
            br_if 2 (;@2;)
            local.get 9
            local.get 12
            i32.store16 offset=50
            local.get 2
            i32.const 184
            i32.add
            local.get 3
            local.get 7
            i32.const 5
            i32.shl
            i32.add
            local.tee 11
            i32.const 64
            i32.add
            i64.load align=1
            i64.store
            local.get 2
            i32.const 192
            i32.add
            local.get 11
            i32.const 72
            i32.add
            i64.load align=1
            i64.store
            local.get 2
            i32.const 200
            i32.add
            local.get 11
            i32.const 80
            i32.add
            i32.load align=1
            i32.store
            local.get 2
            local.get 11
            i32.const 56
            i32.add
            i64.load align=1
            i64.store offset=176
            local.get 7
            i32.const 1
            i32.add
            local.tee 13
            local.get 7
            i32.lt_u
            br_if 2 (;@2;)
            local.get 10
            local.get 13
            i32.sub
            local.tee 14
            local.get 10
            i32.gt_u
            br_if 2 (;@2;)
            local.get 3
            local.get 7
            i32.const 2
            i32.shl
            i32.add
            i32.const 4
            i32.add
            i32.load
            local.set 15
            local.get 11
            i32.const 52
            i32.add
            i32.load align=1
            local.set 16
            local.get 2
            i32.const 40
            i32.add
            local.get 9
            i32.const 52
            i32.add
            local.get 12
            call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17ha96656558948c07cE
            local.get 14
            local.get 2
            i32.load offset=44
            i32.ne
            br_if 2 (;@2;)
            local.get 2
            i32.load offset=40
            local.get 3
            local.get 13
            i32.const 5
            i32.shl
            i32.add
            i32.const 52
            i32.add
            local.get 14
            i32.const 5
            i32.shl
            call $memcpy
            drop
            local.get 2
            i32.const 32
            i32.add
            local.get 9
            i32.const 4
            i32.add
            local.get 12
            call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17hda973abfff99f4dfE
            local.get 14
            local.get 2
            i32.load offset=36
            i32.ne
            br_if 2 (;@2;)
            local.get 2
            i32.load offset=32
            local.get 3
            local.get 13
            i32.const 2
            i32.shl
            i32.add
            i32.const 4
            i32.add
            local.get 14
            i32.const 2
            i32.shl
            call $memcpy
            drop
            local.get 3
            local.get 7
            i32.store16 offset=50
            local.get 2
            i32.const 144
            i32.add
            i32.const 8
            i32.add
            local.tee 7
            local.get 2
            i32.const 176
            i32.add
            i32.const 8
            i32.add
            i64.load
            i64.store
            local.get 2
            i32.const 144
            i32.add
            i32.const 16
            i32.add
            local.tee 11
            local.get 2
            i32.const 176
            i32.add
            i32.const 16
            i32.add
            i64.load
            i64.store
            local.get 2
            i32.const 144
            i32.add
            i32.const 24
            i32.add
            local.tee 14
            local.get 2
            i32.const 176
            i32.add
            i32.const 24
            i32.add
            i32.load
            i32.store
            local.get 2
            local.get 2
            i64.load offset=176
            i64.store offset=144
            local.get 2
            local.get 5
            i32.store offset=184
            local.get 2
            local.get 9
            local.get 3
            local.get 6
            select
            i32.store offset=180
            local.get 2
            i32.const 0
            local.get 8
            local.get 6
            select
            i32.store offset=176
            local.get 2
            i32.const 208
            i32.add
            i32.const 24
            i32.add
            local.get 2
            i32.const 112
            i32.add
            i32.const 24
            i32.add
            i64.load
            i64.store
            local.get 2
            i32.const 208
            i32.add
            i32.const 16
            i32.add
            local.get 2
            i32.const 112
            i32.add
            i32.const 16
            i32.add
            i64.load
            i64.store
            local.get 2
            i32.const 208
            i32.add
            i32.const 8
            i32.add
            local.get 2
            i32.const 112
            i32.add
            i32.const 8
            i32.add
            i64.load
            i64.store
            local.get 2
            local.get 2
            i64.load offset=112
            i64.store offset=208
            local.get 2
            i32.const 176
            i32.add
            local.get 2
            i32.const 208
            i32.add
            local.get 1
            call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hd472ed3c7a763cb1E
            local.set 17
            local.get 2
            i32.const 80
            i32.add
            i32.const 8
            i32.add
            local.get 7
            i64.load
            i64.store
            local.get 2
            i32.const 80
            i32.add
            i32.const 16
            i32.add
            local.get 11
            i64.load
            i64.store
            local.get 2
            i32.const 80
            i32.add
            i32.const 24
            i32.add
            local.get 14
            i32.load
            i32.store
            local.get 2
            local.get 2
            i64.load offset=144
            i64.store offset=80
            br 1 (;@3;)
          end
          local.get 2
          i32.const 208
          i32.add
          i32.const 24
          i32.add
          local.get 0
          i32.const 16
          i32.add
          local.tee 7
          i32.const 24
          i32.add
          i64.load align=1
          i64.store
          local.get 2
          i32.const 208
          i32.add
          i32.const 16
          i32.add
          local.get 7
          i32.const 16
          i32.add
          i64.load align=1
          i64.store
          local.get 2
          i32.const 208
          i32.add
          i32.const 8
          i32.add
          local.get 7
          i32.const 8
          i32.add
          i64.load align=1
          i64.store
          local.get 2
          local.get 7
          i64.load align=1
          i64.store offset=208
          local.get 2
          i32.const 48
          i32.add
          local.get 2
          i32.const 208
          i32.add
          local.get 1
          call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hd472ed3c7a763cb1E
          local.set 17
          local.get 2
          i32.load offset=56
          local.set 16
          local.get 2
          i32.load offset=48
          local.set 8
        end
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.const 10
              i32.le_u
              br_if 0 (;@5;)
              local.get 2
              i32.const 48
              i32.add
              i32.const 24
              i32.add
              local.tee 6
              local.get 2
              i32.const 80
              i32.add
              i32.const 24
              i32.add
              i32.load
              i32.store
              local.get 2
              i32.const 48
              i32.add
              i32.const 16
              i32.add
              local.tee 5
              local.get 2
              i32.const 80
              i32.add
              i32.const 16
              i32.add
              i64.load
              i64.store
              local.get 2
              i32.const 48
              i32.add
              i32.const 8
              i32.add
              local.tee 18
              local.get 2
              i32.const 80
              i32.add
              i32.const 8
              i32.add
              i64.load
              i64.store
              local.get 2
              local.get 2
              i64.load offset=80
              i64.store offset=48
              local.get 2
              i32.const 208
              i32.add
              i32.const 4
              i32.or
              local.set 10
              i32.const 0
              local.set 14
              loop  ;; label = @6
                local.get 3
                i32.load
                local.tee 7
                i32.eqz
                br_if 2 (;@4;)
                local.get 8
                i32.const 1
                i32.add
                local.tee 11
                local.get 8
                i32.lt_u
                br_if 4 (;@2;)
                local.get 2
                local.get 3
                i32.load16_u offset=48
                local.tee 8
                i32.store offset=88
                local.get 2
                local.get 7
                i32.store offset=84
                local.get 2
                local.get 11
                i32.store offset=80
                local.get 11
                i32.const -1
                i32.add
                local.tee 3
                local.get 11
                i32.gt_u
                br_if 4 (;@2;)
                local.get 3
                local.get 14
                i32.ne
                br_if 4 (;@2;)
                block  ;; label = @7
                  local.get 7
                  i32.load16_u offset=50
                  i32.const 11
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 208
                  i32.add
                  local.get 8
                  call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
                  local.get 2
                  i32.load offset=216
                  local.set 19
                  local.get 2
                  i32.load offset=212
                  local.set 20
                  local.get 2
                  i32.load offset=208
                  local.set 3
                  local.get 7
                  i32.load16_u offset=50
                  local.set 21
                  call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17haec4b1edce4a03b0E
                  local.set 8
                  local.get 7
                  i32.load16_u offset=50
                  local.tee 12
                  local.get 3
                  i32.sub
                  local.tee 14
                  local.get 12
                  i32.gt_u
                  br_if 5 (;@2;)
                  local.get 14
                  i32.const -1
                  i32.add
                  local.tee 13
                  local.get 14
                  i32.gt_u
                  br_if 5 (;@2;)
                  local.get 8
                  local.get 13
                  i32.store16 offset=50
                  local.get 2
                  i32.const 176
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 22
                  local.get 7
                  local.get 3
                  i32.const 5
                  i32.shl
                  i32.add
                  local.tee 14
                  i32.const 64
                  i32.add
                  i64.load align=1
                  i64.store
                  local.get 2
                  i32.const 176
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 23
                  local.get 14
                  i32.const 72
                  i32.add
                  i64.load align=1
                  i64.store
                  local.get 2
                  i32.const 176
                  i32.add
                  i32.const 24
                  i32.add
                  local.tee 24
                  local.get 14
                  i32.const 80
                  i32.add
                  i32.load align=1
                  i32.store
                  local.get 2
                  local.get 14
                  i32.const 56
                  i32.add
                  i64.load align=1
                  i64.store offset=176
                  local.get 3
                  i32.const 1
                  i32.add
                  local.tee 1
                  local.get 3
                  i32.lt_u
                  br_if 5 (;@2;)
                  local.get 12
                  local.get 1
                  i32.sub
                  local.tee 4
                  local.get 12
                  i32.gt_u
                  br_if 5 (;@2;)
                  local.get 7
                  local.get 3
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 4
                  i32.add
                  i32.load
                  local.set 12
                  local.get 14
                  i32.const 52
                  i32.add
                  i32.load align=1
                  local.set 25
                  local.get 2
                  i32.const 24
                  i32.add
                  local.get 8
                  i32.const 52
                  i32.add
                  local.get 13
                  call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17ha96656558948c07cE
                  local.get 4
                  local.get 2
                  i32.load offset=28
                  i32.ne
                  br_if 5 (;@2;)
                  local.get 2
                  i32.load offset=24
                  local.get 7
                  local.get 1
                  i32.const 5
                  i32.shl
                  i32.add
                  i32.const 52
                  i32.add
                  local.get 4
                  i32.const 5
                  i32.shl
                  call $memcpy
                  drop
                  local.get 2
                  i32.const 16
                  i32.add
                  local.get 8
                  i32.const 4
                  i32.add
                  local.get 13
                  call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17hda973abfff99f4dfE
                  local.get 4
                  local.get 2
                  i32.load offset=20
                  i32.ne
                  br_if 5 (;@2;)
                  local.get 2
                  i32.load offset=16
                  local.get 7
                  local.get 1
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 13
                  i32.const 4
                  i32.add
                  local.get 4
                  i32.const 2
                  i32.shl
                  call $memcpy
                  drop
                  local.get 7
                  local.get 3
                  i32.store16 offset=50
                  local.get 2
                  i32.const 208
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 4
                  local.get 22
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 208
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 22
                  local.get 23
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 208
                  i32.add
                  i32.const 24
                  i32.add
                  local.tee 23
                  local.get 24
                  i32.load
                  i32.store
                  local.get 2
                  local.get 2
                  i64.load offset=176
                  i64.store offset=208
                  local.get 21
                  i32.const 1
                  i32.add
                  local.tee 14
                  local.get 1
                  i32.sub
                  local.tee 3
                  local.get 14
                  i32.gt_u
                  br_if 5 (;@2;)
                  local.get 8
                  i32.load16_u offset=50
                  local.tee 14
                  i32.const 12
                  i32.ge_u
                  br_if 6 (;@1;)
                  local.get 3
                  local.get 14
                  i32.const 1
                  i32.add
                  i32.ne
                  br_if 5 (;@2;)
                  local.get 8
                  i32.const 404
                  i32.add
                  local.get 13
                  i32.const 404
                  i32.add
                  local.get 3
                  i32.const 2
                  i32.shl
                  call $memcpy
                  drop
                  local.get 2
                  i32.const 8
                  i32.add
                  local.get 8
                  local.get 11
                  call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h09b48f3afe627b6eE
                  local.get 2
                  i32.const 112
                  i32.add
                  i32.const 8
                  i32.add
                  local.tee 13
                  local.get 4
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 112
                  i32.add
                  i32.const 16
                  i32.add
                  local.tee 4
                  local.get 22
                  i64.load
                  i64.store
                  local.get 2
                  i32.const 112
                  i32.add
                  i32.const 24
                  i32.add
                  local.tee 21
                  local.get 23
                  i32.load
                  i32.store
                  local.get 2
                  local.get 2
                  i64.load offset=208
                  i64.store offset=112
                  local.get 2
                  i32.load offset=12
                  local.set 3
                  local.get 2
                  i32.load offset=8
                  local.set 14
                  local.get 11
                  local.set 8
                  local.get 7
                  local.set 1
                  block  ;; label = @8
                    local.get 20
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 14
                    local.set 8
                    local.get 3
                    local.set 1
                  end
                  local.get 2
                  local.get 19
                  i32.store offset=184
                  local.get 2
                  local.get 1
                  i32.store offset=180
                  local.get 2
                  local.get 8
                  i32.store offset=176
                  local.get 10
                  local.get 2
                  i64.load offset=48
                  i64.store align=4
                  local.get 10
                  i32.const 8
                  i32.add
                  local.get 18
                  i64.load
                  i64.store align=4
                  local.get 10
                  i32.const 16
                  i32.add
                  local.get 5
                  i64.load
                  i64.store align=4
                  local.get 10
                  i32.const 24
                  i32.add
                  local.get 6
                  i32.load
                  i32.store
                  local.get 2
                  local.get 16
                  i32.store offset=208
                  local.get 2
                  i32.const 176
                  i32.add
                  local.get 2
                  i32.const 208
                  i32.add
                  local.get 15
                  local.get 9
                  call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hde9b41d070f2143aE
                  local.get 2
                  i32.const 144
                  i32.add
                  i32.const 24
                  i32.add
                  local.get 21
                  i32.load
                  local.tee 8
                  i32.store
                  local.get 2
                  i32.const 144
                  i32.add
                  i32.const 16
                  i32.add
                  local.get 4
                  i64.load
                  local.tee 26
                  i64.store
                  local.get 2
                  i32.const 144
                  i32.add
                  i32.const 8
                  i32.add
                  local.get 13
                  i64.load
                  local.tee 27
                  i64.store
                  local.get 18
                  local.get 27
                  i64.store
                  local.get 5
                  local.get 26
                  i64.store
                  local.get 6
                  local.get 8
                  i32.store
                  local.get 2
                  local.get 2
                  i64.load offset=112
                  local.tee 26
                  i64.store offset=144
                  local.get 2
                  local.get 26
                  i64.store offset=48
                  local.get 11
                  local.set 8
                  local.get 25
                  local.set 16
                  local.get 3
                  local.set 9
                  local.get 12
                  local.set 15
                  local.get 7
                  local.set 3
                  br 1 (;@6;)
                end
              end
              local.get 10
              local.get 2
              i64.load offset=48
              i64.store align=4
              local.get 10
              i32.const 8
              i32.add
              local.get 2
              i32.const 48
              i32.add
              i32.const 8
              i32.add
              i64.load
              i64.store align=4
              local.get 10
              i32.const 16
              i32.add
              local.get 2
              i32.const 48
              i32.add
              i32.const 16
              i32.add
              i64.load
              i64.store align=4
              local.get 10
              i32.const 24
              i32.add
              local.get 2
              i32.const 48
              i32.add
              i32.const 24
              i32.add
              i32.load
              i32.store
              local.get 2
              local.get 16
              i32.store offset=208
              local.get 2
              i32.const 80
              i32.add
              local.get 2
              i32.const 208
              i32.add
              local.get 15
              local.get 9
              call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hde9b41d070f2143aE
            end
            local.get 0
            i32.load offset=12
            local.tee 3
            i32.load offset=8
            local.tee 11
            i32.const 1
            i32.add
            local.tee 7
            local.get 11
            i32.lt_u
            br_if 2 (;@2;)
            local.get 3
            i32.const 8
            i32.add
            local.set 11
            br 1 (;@3;)
          end
          local.get 2
          i32.const 112
          i32.add
          i32.const 24
          i32.add
          local.get 2
          i32.const 48
          i32.add
          i32.const 24
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 112
          i32.add
          i32.const 16
          i32.add
          local.get 2
          i32.const 48
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 112
          i32.add
          i32.const 8
          i32.add
          local.get 2
          i32.const 48
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 2
          i64.load offset=48
          i64.store offset=112
          local.get 0
          i32.load offset=12
          local.tee 11
          i32.load offset=4
          local.tee 3
          i32.eqz
          br_if 1 (;@2;)
          local.get 11
          i32.load
          local.set 7
          call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17haec4b1edce4a03b0E
          local.tee 8
          local.get 3
          i32.store offset=404
          local.get 7
          i32.const 1
          i32.add
          local.tee 3
          local.get 7
          i32.lt_u
          br_if 1 (;@2;)
          local.get 2
          local.get 8
          local.get 3
          call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h09b48f3afe627b6eE
          local.get 2
          i32.load
          local.set 3
          local.get 11
          local.get 2
          i32.load offset=4
          local.tee 7
          i32.store offset=4
          local.get 11
          local.get 3
          i32.store
          local.get 2
          i32.const 208
          i32.add
          i32.const 24
          i32.add
          local.get 2
          i32.const 112
          i32.add
          i32.const 24
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 208
          i32.add
          i32.const 16
          i32.add
          local.get 2
          i32.const 112
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 208
          i32.add
          i32.const 8
          i32.add
          local.get 2
          i32.const 112
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 2
          i64.load offset=112
          i64.store offset=208
          local.get 3
          i32.const -1
          i32.add
          local.tee 8
          local.get 3
          i32.gt_u
          br_if 1 (;@2;)
          local.get 8
          local.get 14
          i32.ne
          br_if 1 (;@2;)
          local.get 7
          i32.load16_u offset=50
          local.tee 14
          i32.const 10
          i32.gt_u
          br_if 1 (;@2;)
          local.get 7
          local.get 14
          i32.const 5
          i32.shl
          i32.add
          local.tee 8
          i32.const 52
          i32.add
          local.get 16
          i32.store align=1
          local.get 7
          local.get 14
          i32.const 1
          i32.add
          local.tee 1
          i32.store16 offset=50
          local.get 8
          i32.const 56
          i32.add
          local.get 2
          i64.load offset=208
          i64.store align=1
          local.get 8
          i32.const 64
          i32.add
          local.get 2
          i32.const 208
          i32.add
          i32.const 8
          i32.add
          i64.load
          i64.store align=1
          local.get 8
          i32.const 72
          i32.add
          local.get 2
          i32.const 224
          i32.add
          i64.load
          i64.store align=1
          local.get 8
          i32.const 80
          i32.add
          local.get 2
          i32.const 232
          i32.add
          i32.load
          i32.store align=1
          local.get 7
          local.get 14
          i32.const 2
          i32.shl
          i32.add
          i32.const 4
          i32.add
          local.get 15
          i32.store
          local.get 7
          local.get 1
          i32.const 2
          i32.shl
          i32.add
          i32.const 404
          i32.add
          local.get 9
          i32.store
          local.get 2
          local.get 1
          i32.store offset=184
          local.get 2
          local.get 7
          i32.store offset=180
          local.get 2
          local.get 3
          i32.store offset=176
          local.get 2
          i32.const 176
          i32.add
          call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17hee3182ae6ccf1120E
          local.get 11
          i32.load offset=8
          local.tee 3
          i32.const 1
          i32.add
          local.tee 7
          local.get 3
          i32.lt_u
          br_if 1 (;@2;)
          local.get 11
          i32.const 8
          i32.add
          local.set 11
        end
        local.get 11
        local.get 7
        i32.store
        local.get 2
        i32.const 240
        i32.add
        global.set 0
        local.get 17
        return
      end
      unreachable
      unreachable
    end
    call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
    unreachable)
  (func $_ZN11ink_storage4lazy13Lazy$LT$T$GT$3get17haf22a627b7e19edaE (type 7) (param i32) (result i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 112
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          i32.load offset=40
          local.tee 2
          i32.const 2
          i32.ne
          br_if 0 (;@3;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 0
              i64.load
              i64.const 1
              i64.ne
              br_if 0 (;@5;)
              local.get 1
              i32.const 8
              i32.add
              i32.const 24
              i32.add
              local.tee 2
              local.get 0
              i32.const 32
              i32.add
              i64.load
              i64.store
              local.get 1
              i32.const 8
              i32.add
              i32.const 16
              i32.add
              local.tee 3
              local.get 0
              i32.const 24
              i32.add
              i64.load
              i64.store
              local.get 1
              i32.const 8
              i32.add
              i32.const 8
              i32.add
              local.tee 4
              local.get 0
              i32.const 16
              i32.add
              i64.load
              i64.store
              local.get 1
              local.get 0
              i64.load offset=8
              i64.store offset=8
              local.get 1
              i32.const 8
              i32.add
              call $_ZN7ink_env3api20get_contract_storage17hed1fdc7f0b1449efE
              i32.const 1
              i32.and
              i32.eqz
              br_if 0 (;@5;)
              local.get 1
              i32.const 40
              i32.add
              i32.const 24
              i32.add
              local.get 2
              i64.load
              i64.store
              local.get 1
              i32.const 40
              i32.add
              i32.const 16
              i32.add
              local.get 3
              i64.load
              i64.store
              local.get 1
              i32.const 40
              i32.add
              i32.const 8
              i32.add
              local.get 4
              i64.load
              i64.store
              local.get 1
              local.get 1
              i64.load offset=8
              i64.store offset=40
              local.get 1
              i64.const 0
              i64.store offset=72
              i32.const 0
              local.set 3
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 1
                        i32.const 40
                        i32.add
                        call $_ZN11ink_storage6traits5impls5prims1_74_$LT$impl$u20$ink_storage..traits..spread..SpreadLayout$u20$for$u20$u8$GT$11pull_spread17he441713b4b29dc32E
                        i32.const 255
                        i32.and
                        br_table 3 (;@7;) 0 (;@10;) 1 (;@9;)
                      end
                      local.get 1
                      i32.const 40
                      i32.add
                      call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hf1aaf24072cf789cE
                      local.set 2
                      local.get 1
                      i32.const 16384
                      i32.store offset=84
                      local.get 1
                      i32.const 68528
                      i32.store offset=80
                      local.get 2
                      local.get 1
                      i32.const 80
                      i32.add
                      call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
                      local.tee 2
                      i32.const 13
                      i32.eq
                      br_if 1 (;@8;)
                      local.get 2
                      i32.const 3
                      i32.eq
                      br_if 7 (;@2;)
                    end
                    unreachable
                    unreachable
                  end
                  local.get 1
                  local.get 1
                  i64.load offset=80
                  i64.store offset=104
                  local.get 1
                  i32.const 88
                  i32.add
                  local.get 1
                  i32.const 104
                  i32.add
                  call $_ZN75_$LT$alloc..string..String$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4e2a95228ac42f1aE
                  local.get 1
                  i32.load offset=88
                  local.tee 3
                  i32.eqz
                  br_if 1 (;@6;)
                  local.get 1
                  i64.load offset=92 align=4
                  local.set 5
                end
                i32.const 1
                local.set 2
                br 2 (;@4;)
              end
              call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
              unreachable
            end
            i32.const 0
            local.set 2
          end
          local.get 0
          local.get 2
          i32.store offset=40
          local.get 0
          i32.const 56
          i32.add
          i32.const 1
          i32.store8
          local.get 0
          i32.const 48
          i32.add
          local.get 5
          i64.store
          local.get 0
          i32.const 44
          i32.add
          local.get 3
          i32.store
        end
        local.get 2
        i32.const 1
        i32.eq
        br_if 1 (;@1;)
      end
      call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
      unreachable
    end
    local.get 1
    i32.const 112
    i32.add
    global.set 0
    local.get 0
    i32.const 40
    i32.add
    i32.const 4
    i32.add)
  (func $_ZN7ink_env3api20get_contract_storage17hed1fdc7f0b1449efE (type 7) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 16384
    i32.store offset=12
    local.get 1
    i32.const 68528
    i32.store offset=8
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          local.get 1
          i32.const 8
          i32.add
          call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
          local.tee 0
          i32.const 3
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          i32.const 13
          i32.eq
          br_if 1 (;@2;)
          unreachable
          unreachable
        end
        i32.const 0
        local.set 0
        br 1 (;@1;)
      end
      i32.const 1
      local.set 0
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN11ink_storage6traits5impls5prims1_74_$LT$impl$u20$ink_storage..traits..spread..SpreadLayout$u20$for$u20$u8$GT$11pull_spread17he441713b4b29dc32E (type 7) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hfb57d7af5e666bbcE
    local.set 0
    local.get 1
    i32.const 16384
    i32.store offset=20
    local.get 1
    i32.const 68528
    i32.store offset=16
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          local.get 1
          i32.const 16
          i32.add
          call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
          local.tee 0
          i32.const 13
          i32.eq
          br_if 0 (;@3;)
          local.get 0
          i32.const 3
          i32.eq
          br_if 1 (;@2;)
          unreachable
          unreachable
        end
        local.get 1
        local.get 1
        i64.load offset=16
        i64.store offset=24
        local.get 1
        i32.const 8
        i32.add
        local.get 1
        i32.const 24
        i32.add
        call $_ZN56_$LT$u8$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h374e211fca1804deE
        local.get 1
        i32.load8_u offset=8
        i32.const 1
        i32.and
        i32.eqz
        br_if 1 (;@1;)
        call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
        unreachable
      end
      call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
      unreachable
    end
    local.get 1
    i32.load8_u offset=9
    local.set 0
    local.get 1
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hf1aaf24072cf789cE (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.load offset=4
    i32.store offset=12
    local.get 0
    local.get 1
    i32.load
    local.get 2
    i32.const 12
    i32.add
    call $_ZN7ink_env6engine8on_chain3ext3sys16seal_get_storage17h26468309f0d4e343E
    local.set 0
    local.get 1
    local.get 2
    i32.load offset=12
    call $_ZN7ink_env6engine8on_chain3ext18extract_from_slice17h8e422b55d0a95429E
    local.get 0
    call $_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h20933c84fa909027E
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN75_$LT$alloc..string..String$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4e2a95228ac42f1aE (type 4) (param i32 i32)
    (local i32 i32 i64 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    call $_ZN78_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17hf1fcdcd6d2b09f13E
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        i32.const 0
        local.get 2
        i64.load offset=4 align=4
        local.tee 4
        i64.const 32
        i64.shr_u
        i32.wrap_i64
        local.tee 5
        i32.const -7
        i32.add
        local.tee 1
        local.get 1
        local.get 5
        i32.gt_u
        select
        local.set 6
        local.get 3
        i32.const 3
        i32.add
        i32.const -4
        i32.and
        local.get 3
        i32.sub
        local.set 7
        i32.const 0
        local.set 1
        block  ;; label = @3
          loop  ;; label = @4
            local.get 1
            local.get 5
            i32.ge_u
            br_if 1 (;@3;)
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 3
                    local.get 1
                    i32.add
                    i32.load8_u
                    local.tee 8
                    i32.const 24
                    i32.shl
                    i32.const 24
                    i32.shr_s
                    local.tee 9
                    i32.const 0
                    i32.lt_s
                    br_if 0 (;@8;)
                    local.get 7
                    i32.const -1
                    i32.eq
                    br_if 3 (;@5;)
                    local.get 7
                    local.get 1
                    i32.sub
                    i32.const 3
                    i32.and
                    br_if 3 (;@5;)
                    loop  ;; label = @9
                      local.get 1
                      local.get 6
                      i32.ge_u
                      br_if 3 (;@6;)
                      local.get 3
                      local.get 1
                      i32.add
                      local.tee 8
                      i32.const 4
                      i32.add
                      i32.load
                      local.get 8
                      i32.load
                      i32.or
                      i32.const -2139062144
                      i32.and
                      br_if 3 (;@6;)
                      local.get 1
                      i32.const 8
                      i32.add
                      local.tee 8
                      local.get 1
                      i32.lt_u
                      br_if 2 (;@7;)
                      local.get 8
                      local.set 1
                      br 0 (;@9;)
                    end
                  end
                  i32.const 0
                  local.set 10
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 8
                          i32.const 65826
                          i32.add
                          i32.load8_u
                          i32.const -2
                          i32.add
                          br_table 0 (;@11;) 2 (;@9;) 1 (;@10;) 10 (;@1;)
                        end
                        local.get 1
                        i32.const 1
                        i32.add
                        local.tee 1
                        local.get 5
                        i32.ge_u
                        br_if 9 (;@1;)
                        local.get 3
                        local.get 1
                        i32.add
                        i32.load8_u
                        i32.const 192
                        i32.and
                        i32.const 128
                        i32.eq
                        br_if 2 (;@8;)
                        br 9 (;@1;)
                      end
                      local.get 1
                      i32.const 1
                      i32.add
                      local.tee 11
                      local.get 5
                      i32.ge_u
                      br_if 8 (;@1;)
                      local.get 3
                      local.get 11
                      i32.add
                      i32.load8_u
                      local.set 11
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 8
                              i32.const -240
                              i32.add
                              br_table 1 (;@12;) 0 (;@13;) 0 (;@13;) 0 (;@13;) 2 (;@11;) 0 (;@13;)
                            end
                            local.get 9
                            i32.const 15
                            i32.add
                            i32.const 255
                            i32.and
                            i32.const 2
                            i32.gt_u
                            br_if 11 (;@1;)
                            local.get 11
                            i32.const 24
                            i32.shl
                            i32.const 24
                            i32.shr_s
                            i32.const -1
                            i32.gt_s
                            br_if 11 (;@1;)
                            local.get 11
                            i32.const 255
                            i32.and
                            i32.const 192
                            i32.lt_u
                            br_if 2 (;@10;)
                            br 11 (;@1;)
                          end
                          local.get 11
                          i32.const 112
                          i32.add
                          i32.const 255
                          i32.and
                          i32.const 48
                          i32.lt_u
                          br_if 1 (;@10;)
                          br 10 (;@1;)
                        end
                        local.get 11
                        i32.const 24
                        i32.shl
                        i32.const 24
                        i32.shr_s
                        i32.const -1
                        i32.gt_s
                        br_if 9 (;@1;)
                        local.get 11
                        i32.const 255
                        i32.and
                        i32.const 143
                        i32.gt_u
                        br_if 9 (;@1;)
                      end
                      local.get 1
                      i32.const 2
                      i32.add
                      local.tee 8
                      local.get 5
                      i32.ge_u
                      br_if 8 (;@1;)
                      local.get 3
                      local.get 8
                      i32.add
                      i32.load8_u
                      i32.const 192
                      i32.and
                      i32.const 128
                      i32.ne
                      br_if 8 (;@1;)
                      local.get 1
                      i32.const 3
                      i32.add
                      local.tee 1
                      local.get 5
                      i32.ge_u
                      br_if 8 (;@1;)
                      local.get 3
                      local.get 1
                      i32.add
                      i32.load8_u
                      i32.const 192
                      i32.and
                      i32.const 128
                      i32.eq
                      br_if 1 (;@8;)
                      br 8 (;@1;)
                    end
                    local.get 1
                    i32.const 1
                    i32.add
                    local.tee 11
                    local.get 5
                    i32.ge_u
                    br_if 7 (;@1;)
                    local.get 3
                    local.get 11
                    i32.add
                    i32.load8_u
                    local.set 11
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 8
                            i32.const 224
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 8
                            i32.const 237
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 9
                            i32.const 31
                            i32.add
                            i32.const 255
                            i32.and
                            i32.const 12
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 9
                            i32.const -2
                            i32.and
                            i32.const -18
                            i32.ne
                            br_if 11 (;@1;)
                            local.get 11
                            i32.const 24
                            i32.shl
                            i32.const 24
                            i32.shr_s
                            i32.const -1
                            i32.gt_s
                            br_if 11 (;@1;)
                            local.get 11
                            i32.const 255
                            i32.and
                            i32.const 192
                            i32.lt_u
                            br_if 3 (;@9;)
                            br 11 (;@1;)
                          end
                          local.get 11
                          i32.const 224
                          i32.and
                          i32.const 160
                          i32.eq
                          br_if 2 (;@9;)
                          br 10 (;@1;)
                        end
                        local.get 11
                        i32.const 24
                        i32.shl
                        i32.const 24
                        i32.shr_s
                        i32.const -1
                        i32.gt_s
                        br_if 9 (;@1;)
                        local.get 11
                        i32.const 255
                        i32.and
                        i32.const 160
                        i32.lt_u
                        br_if 1 (;@9;)
                        br 9 (;@1;)
                      end
                      local.get 11
                      i32.const 24
                      i32.shl
                      i32.const 24
                      i32.shr_s
                      i32.const -1
                      i32.gt_s
                      br_if 8 (;@1;)
                      local.get 11
                      i32.const 255
                      i32.and
                      i32.const 191
                      i32.gt_u
                      br_if 8 (;@1;)
                    end
                    local.get 1
                    i32.const 2
                    i32.add
                    local.tee 1
                    local.get 5
                    i32.ge_u
                    br_if 7 (;@1;)
                    local.get 3
                    local.get 1
                    i32.add
                    i32.load8_u
                    i32.const 192
                    i32.and
                    i32.const 128
                    i32.ne
                    br_if 7 (;@1;)
                  end
                  local.get 1
                  i32.const 1
                  i32.add
                  local.set 1
                  br 3 (;@4;)
                end
                unreachable
                unreachable
              end
              local.get 1
              local.get 5
              local.get 1
              local.get 5
              i32.gt_u
              select
              local.set 8
              loop  ;; label = @6
                block  ;; label = @7
                  local.get 8
                  local.get 1
                  i32.ne
                  br_if 0 (;@7;)
                  local.get 8
                  local.set 1
                  br 3 (;@4;)
                end
                local.get 3
                local.get 1
                i32.add
                i32.load8_s
                i32.const 0
                i32.lt_s
                br_if 2 (;@4;)
                local.get 1
                i32.const 1
                i32.add
                local.set 1
                br 0 (;@6;)
              end
            end
            local.get 1
            i32.const 1
            i32.add
            local.set 1
            br 0 (;@4;)
          end
        end
        local.get 0
        local.get 4
        i64.store offset=4 align=4
        local.get 3
        local.set 10
        br 1 (;@1;)
      end
      i32.const 0
      local.set 10
    end
    local.get 0
    local.get 10
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E (type 8)
    unreachable
    unreachable)
  (func $_ZN11ink_storage4lazy13Lazy$LT$T$GT$3set17hc4bdacba33cb93cdE (type 4) (param i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=40
        i32.const 2
        i32.eq
        local.tee 2
        br_if 0 (;@2;)
        i32.const 0
        local.get 0
        i32.const 40
        i32.add
        local.get 2
        select
        local.tee 0
        i32.const 1
        i32.store
        local.get 0
        local.get 1
        i64.load align=4
        i64.store offset=4 align=4
        local.get 0
        i32.const 12
        i32.add
        local.get 1
        i32.const 8
        i32.add
        i32.load
        i32.store
        local.get 0
        i32.const 16
        i32.add
        local.set 0
        br 1 (;@1;)
      end
      local.get 0
      i32.const 1
      i32.store offset=40
      local.get 0
      i32.const 44
      i32.add
      local.get 1
      i64.load align=4
      i64.store align=4
      local.get 0
      i32.const 52
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i32.load
      i32.store
      local.get 0
      i32.const 56
      i32.add
      local.set 0
    end
    local.get 0
    i32.const 0
    i32.store8)
  (func $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$3put17hc4decdb7e8d67aaeE (type 2) (param i32 i32 i32)
    (local i32)
    local.get 2
    i32.load8_u
    local.set 3
    local.get 0
    local.get 1
    i32.const 68
    call $memcpy
    local.set 0
    local.get 1
    local.get 2
    i32.const 68
    call $memcpy
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u
        i32.const 2
        i32.ne
        br_if 0 (;@2;)
        local.get 3
        i32.const 255
        i32.and
        i32.const 2
        i32.eq
        br_if 1 (;@1;)
      end
      local.get 1
      i32.const 0
      i32.store8 offset=68
    end)
  (func $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$7get_mut17h6f3f997cff70f2ecE (type 1) (param i32 i32) (result i32)
    (local i32)
    i32.const 0
    local.set 2
    block  ;; label = @1
      local.get 0
      local.get 1
      call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$11lazily_load17h5239fbf66e51cb1fE
      local.tee 0
      i32.load8_u
      i32.const 2
      i32.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store8 offset=68
      local.get 0
      local.set 2
    end
    local.get 2)
  (func $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$11lazily_load17h5239fbf66e51cb1fE (type 1) (param i32 i32) (result i32)
    (local i32 i32 i64 i64 i64 i64 i64 i32 i32)
    global.get 0
    i32.const 288
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 16
    i32.add
    local.get 0
    i32.const 40
    i32.add
    local.get 1
    call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17hfb07304c0de55b1cE
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=16
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          i32.const 2
          local.set 3
          block  ;; label = @4
            local.get 0
            i64.load
            i64.const 1
            i64.ne
            br_if 0 (;@4;)
            local.get 0
            i32.const 32
            i32.add
            i64.load
            local.set 4
            local.get 0
            i32.const 24
            i32.add
            i64.load
            local.set 5
            local.get 0
            i32.const 16
            i32.add
            i64.load
            local.set 6
            local.get 2
            local.get 0
            i64.load offset=8
            local.tee 7
            local.get 1
            i64.extend_i32_u
            i64.add
            local.tee 8
            i64.store offset=40
            local.get 2
            local.get 6
            local.get 8
            local.get 7
            i64.lt_u
            i64.extend_i32_u
            i64.add
            local.tee 7
            i64.store offset=48
            local.get 2
            local.get 5
            local.get 7
            local.get 6
            i64.lt_u
            i64.extend_i32_u
            i64.add
            local.tee 6
            i64.store offset=56
            local.get 2
            local.get 4
            local.get 6
            local.get 5
            i64.lt_u
            i64.extend_i32_u
            i64.add
            i64.store offset=64
            local.get 2
            i32.const 16384
            i32.store offset=132
            local.get 2
            i32.const 68528
            i32.store offset=128
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 40
                i32.add
                local.get 2
                i32.const 128
                i32.add
                call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
                local.tee 0
                i32.const 3
                i32.eq
                br_if 0 (;@6;)
                local.get 0
                i32.const 13
                i32.ne
                br_if 5 (;@1;)
                local.get 2
                local.get 2
                i64.load offset=128
                i64.store offset=200
                local.get 2
                i32.const 8
                i32.add
                local.get 2
                i32.const 200
                i32.add
                call $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E
                block  ;; label = @7
                  local.get 2
                  i32.load8_u offset=8
                  i32.const 1
                  i32.and
                  br_if 0 (;@7;)
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 2
                        i32.load8_u offset=9
                        i32.const 255
                        i32.and
                        br_table 0 (;@10;) 1 (;@9;) 3 (;@7;)
                      end
                      local.get 2
                      i32.const 72
                      i32.add
                      local.get 2
                      i32.const 200
                      i32.add
                      call $_ZN11ink_storage11collections5stash1_108_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_storage..collections..stash..VacantEntry$GT$6decode17heeaed37a09162c5eE
                      local.get 2
                      i32.load offset=72
                      i32.const 1
                      i32.eq
                      br_if 2 (;@7;)
                      local.get 2
                      i32.const 80
                      i32.add
                      i32.load
                      local.set 9
                      local.get 2
                      i32.load offset=76
                      local.set 10
                      i32.const 0
                      local.set 3
                      br 1 (;@8;)
                    end
                    local.get 2
                    i32.const 208
                    i32.add
                    local.get 2
                    i32.const 200
                    i32.add
                    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                    local.get 2
                    i32.load8_u offset=208
                    i32.const 1
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 2
                    i32.const 248
                    i32.add
                    local.get 2
                    i32.const 200
                    i32.add
                    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                    local.get 2
                    i32.load8_u offset=248
                    i32.const 1
                    i32.eq
                    br_if 1 (;@7;)
                    local.get 2
                    i32.const 198
                    i32.add
                    local.get 2
                    i32.load8_u offset=211
                    i32.store8
                    local.get 2
                    i32.const 117
                    i32.add
                    local.get 2
                    i32.const 248
                    i32.add
                    i32.const 25
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 2
                    i32.const 109
                    i32.add
                    local.get 2
                    i32.const 265
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 2
                    i32.const 101
                    i32.add
                    local.get 2
                    i32.const 257
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 2
                    i32.const 72
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 2
                    i32.const 228
                    i32.add
                    i64.load align=4
                    i64.store
                    local.get 2
                    i32.const 85
                    i32.add
                    local.get 2
                    i32.const 208
                    i32.add
                    i32.const 25
                    i32.add
                    i64.load align=1
                    i64.store align=1
                    local.get 2
                    local.get 2
                    i64.load offset=249 align=1
                    i64.store offset=93 align=1
                    local.get 2
                    local.get 2
                    i32.load16_u offset=209 align=1
                    i32.store16 offset=196
                    local.get 2
                    local.get 2
                    i32.const 220
                    i32.add
                    i64.load align=4
                    i64.store offset=72
                    local.get 2
                    i32.const 208
                    i32.add
                    i32.const 8
                    i32.add
                    i32.load
                    local.set 9
                    local.get 2
                    i32.load offset=212
                    local.set 10
                    local.get 2
                    i32.const 140
                    i32.add
                    local.get 2
                    i32.const 72
                    i32.add
                    i32.const 53
                    call $memcpy
                    drop
                    i32.const 1
                    local.set 3
                  end
                  local.get 2
                  i32.const 248
                  i32.add
                  i32.const 2
                  i32.add
                  local.get 2
                  i32.const 196
                  i32.add
                  i32.const 2
                  i32.add
                  i32.load8_u
                  i32.store8
                  local.get 2
                  local.get 2
                  i32.load16_u offset=196
                  i32.store16 offset=248
                  local.get 2
                  i32.const 72
                  i32.add
                  local.get 2
                  i32.const 140
                  i32.add
                  i32.const 56
                  call $memcpy
                  drop
                  br 2 (;@5;)
                end
                call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
                unreachable
              end
              i32.const 2
              local.set 3
            end
            local.get 2
            i32.const 208
            i32.add
            i32.const 2
            i32.add
            local.tee 1
            local.get 2
            i32.const 248
            i32.add
            i32.const 2
            i32.add
            local.tee 0
            i32.load8_u
            i32.store8
            local.get 2
            local.get 2
            i32.load16_u offset=248
            i32.store16 offset=208
            local.get 2
            i32.const 140
            i32.add
            local.get 2
            i32.const 72
            i32.add
            i32.const 56
            call $memcpy
            drop
            block  ;; label = @5
              local.get 3
              i32.const 2
              i32.eq
              br_if 0 (;@5;)
              local.get 0
              local.get 1
              i32.load8_u
              i32.store8
              local.get 2
              local.get 2
              i32.load16_u offset=208
              i32.store16 offset=248
              local.get 2
              i32.const 72
              i32.add
              local.get 2
              i32.const 140
              i32.add
              i32.const 56
              call $memcpy
              drop
            end
            local.get 2
            i32.const 200
            i32.add
            i32.const 2
            i32.add
            local.get 0
            i32.load8_u
            i32.store8
            local.get 2
            local.get 2
            i32.load16_u offset=248
            i32.store16 offset=200
            local.get 2
            i32.const 140
            i32.add
            local.get 2
            i32.const 72
            i32.add
            i32.const 56
            call $memcpy
            drop
          end
          local.get 2
          i32.const 72
          i32.add
          i32.const 16
          i32.add
          local.get 2
          i32.const 16
          i32.add
          i32.const 4
          i32.or
          local.tee 0
          i32.const 16
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 72
          i32.add
          i32.const 8
          i32.add
          local.get 0
          i32.const 8
          i32.add
          i64.load align=4
          i64.store
          local.get 2
          local.get 0
          i64.load align=4
          i64.store offset=72
          i32.const 72
          i32.const 4
          call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E.240
          local.tee 0
          local.get 3
          i32.store8
          local.get 0
          local.get 9
          i32.store offset=8 align=1
          local.get 0
          local.get 10
          i32.store offset=4 align=1
          local.get 0
          local.get 2
          i32.load16_u offset=200
          i32.store16 offset=1 align=1
          local.get 0
          i32.const 3
          i32.add
          local.get 2
          i32.const 202
          i32.add
          i32.load8_u
          i32.store8
          local.get 0
          i32.const 12
          i32.add
          local.get 2
          i32.const 140
          i32.add
          i32.const 56
          call $memcpy
          drop
          local.get 0
          i32.const 1
          i32.store8 offset=68
          local.get 2
          i32.const 72
          i32.add
          local.get 0
          call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17h4e24b0c0e3e4d4f8E
          i32.load
          local.set 0
          br 1 (;@2;)
        end
        local.get 2
        i32.const 24
        i32.add
        i32.load
        local.get 2
        i32.const 28
        i32.add
        i32.load
        i32.const 2
        i32.shl
        i32.add
        i32.const 48
        i32.add
        i32.load
        local.set 0
      end
      local.get 2
      i32.const 288
      i32.add
      global.set 0
      local.get 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E (type 4) (param i32 i32)
    (local i32 i64 i64 i64 i64)
    global.get 0
    i32.const 160
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 2
    i64.const 0
    i64.store offset=32
    local.get 2
    local.get 1
    i64.load
    i64.store
    local.get 0
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h816f7eb6f73483c5E
    local.tee 1
    i64.load
    i64.store offset=8
    local.get 0
    i32.const 32
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 72
    i32.add
    local.get 2
    call $_ZN94_$LT$ink_storage..pack..Pack$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11pull_spread17hd1d25f300da86c76E
    local.get 0
    i32.const 136
    i32.add
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h292c2952bec8a29dE
    local.tee 1
    i64.load
    i64.store
    local.get 0
    i32.const 160
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 152
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 144
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 172
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 128
    i32.add
    i64.const 1
    i64.store
    local.get 0
    i32.const 192
    i32.add
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h0af670e7d6bdc4d8E
    local.tee 1
    i64.load
    i64.store
    local.get 0
    i32.const 216
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 208
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 200
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 228
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 184
    i32.add
    i64.const 1
    i64.store
    local.get 0
    i32.const 240
    i32.add
    local.get 2
    call $_ZN94_$LT$ink_storage..pack..Pack$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11pull_spread17hd1d25f300da86c76E
    local.get 0
    i32.const 304
    i32.add
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h2846b24ec5632223E
    local.tee 1
    i64.load
    i64.store
    local.get 0
    i32.const 328
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 320
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 312
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 340
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 296
    i32.add
    i64.const 1
    i64.store
    local.get 0
    i32.const 360
    i32.add
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h5e1376fde4a99d56E
    local.tee 1
    i64.load
    i64.store
    local.get 0
    i32.const 384
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 376
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 368
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 396
    i32.add
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 352
    i32.add
    i64.const 1
    i64.store
    local.get 0
    i64.const 2
    i64.store offset=40
    local.get 0
    i64.const 1
    i64.store
    local.get 0
    i32.const 416
    i32.add
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h8816fe1a954ed65bE
    local.tee 1
    i64.load
    i64.store
    local.get 0
    i32.const 440
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 432
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 424
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 480
    i32.add
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h8816fe1a954ed65bE
    local.tee 1
    i64.load
    i64.store
    local.get 0
    i32.const 504
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 496
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 488
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 544
    i32.add
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h868bc58131ae9d95E
    local.tee 1
    i64.load
    i64.store
    local.get 1
    i32.const 8
    i32.add
    i64.load
    local.set 3
    local.get 1
    i32.const 16
    i32.add
    i64.load
    local.set 4
    local.get 1
    i32.const 24
    i32.add
    i64.load
    local.set 5
    local.get 0
    i32.const 578
    i32.add
    i32.const 0
    i32.store8
    local.get 0
    i32.const 576
    i32.add
    i32.const 2
    i32.store16
    local.get 0
    i32.const 568
    i32.add
    local.get 5
    i64.store
    local.get 0
    i32.const 560
    i32.add
    local.get 4
    i64.store
    local.get 0
    i32.const 552
    i32.add
    local.get 3
    i64.store
    local.get 0
    i32.const 536
    i32.add
    i64.const 1
    i64.store
    local.get 0
    i32.const 512
    i32.add
    i32.const 2
    i32.store
    local.get 0
    i32.const 472
    i32.add
    i64.const 1
    i64.store
    local.get 0
    i32.const 448
    i32.add
    i32.const 2
    i32.store
    local.get 0
    i64.const 1
    i64.store offset=408
    local.get 2
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h93b0b6e2672b5543E
    local.set 1
    local.get 2
    i32.const 16384
    i32.store offset=108
    local.get 2
    i32.const 68528
    i32.store offset=104
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          local.get 2
          i32.const 104
          i32.add
          call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
          local.tee 1
          i32.const 13
          i32.eq
          br_if 0 (;@3;)
          local.get 1
          i32.const 3
          i32.eq
          br_if 1 (;@2;)
          unreachable
          unreachable
        end
        local.get 2
        local.get 2
        i64.load offset=104
        i64.store offset=152
        local.get 2
        i32.const 112
        i32.add
        local.get 2
        i32.const 152
        i32.add
        call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
        local.get 2
        i32.load8_u offset=112
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
        unreachable
      end
      local.get 2
      i32.const 40
      i32.add
      i32.const 8
      i32.add
      local.get 2
      i32.const 72
      i32.add
      i32.const 8
      i32.add
      i64.load align=1
      i64.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 16
      i32.add
      local.get 2
      i32.const 72
      i32.add
      i32.const 16
      i32.add
      i64.load align=1
      i64.store
      local.get 2
      i32.const 40
      i32.add
      i32.const 24
      i32.add
      local.get 2
      i32.const 72
      i32.add
      i32.const 24
      i32.add
      i64.load align=1
      i64.store
      local.get 2
      local.get 2
      i64.load offset=72 align=1
      i64.store offset=40
      call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
      unreachable
    end
    local.get 2
    i32.const 72
    i32.add
    i32.const 24
    i32.add
    local.get 2
    i32.const 137
    i32.add
    i64.load align=1
    local.tee 3
    i64.store
    local.get 2
    i32.const 48
    i32.add
    local.get 2
    i32.const 121
    i32.add
    i64.load align=1
    local.tee 4
    i64.store
    local.get 2
    i32.const 56
    i32.add
    local.get 2
    i32.const 129
    i32.add
    i64.load align=1
    local.tee 5
    i64.store
    local.get 2
    i32.const 40
    i32.add
    i32.const 24
    i32.add
    local.get 3
    i64.store
    local.get 2
    local.get 2
    i64.load offset=113 align=1
    local.tee 6
    i64.store offset=40
    local.get 0
    i32.const 608
    i32.add
    local.get 3
    i64.store align=1
    local.get 0
    i32.const 600
    i32.add
    local.get 5
    i64.store align=1
    local.get 0
    i32.const 592
    i32.add
    local.get 4
    i64.store align=1
    local.get 0
    local.get 6
    i64.store offset=584 align=1
    local.get 2
    i32.const 160
    i32.add
    global.set 0)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h816f7eb6f73483c5E (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN94_$LT$ink_storage..pack..Pack$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11pull_spread17hd1d25f300da86c76E (type 4) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17ha548bed9e797b95bE
    local.set 3
    local.get 2
    i32.const 16384
    i32.store offset=36
    local.get 2
    i32.const 68528
    i32.store offset=32
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 2
          i32.const 32
          i32.add
          call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
          local.tee 3
          i32.const 13
          i32.eq
          br_if 0 (;@3;)
          local.get 3
          i32.const 3
          i32.eq
          br_if 1 (;@2;)
          unreachable
          unreachable
        end
        local.get 2
        local.get 2
        i64.load offset=32
        i64.store offset=40
        local.get 2
        i32.const 24
        i32.add
        local.get 2
        i32.const 40
        i32.add
        call $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E
        block  ;; label = @3
          local.get 2
          i32.load offset=24
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=28
          local.set 3
          local.get 2
          i32.const 16
          i32.add
          local.get 2
          i32.const 40
          i32.add
          call $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E
          local.get 2
          i32.load offset=16
          br_if 0 (;@3;)
          local.get 2
          i32.load offset=20
          local.set 4
          local.get 2
          i32.const 8
          i32.add
          local.get 2
          i32.const 40
          i32.add
          call $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E
          local.get 2
          i32.load offset=8
          i32.eqz
          br_if 2 (;@1;)
        end
        call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
        unreachable
      end
      call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
      unreachable
    end
    local.get 0
    i32.const 48
    i32.add
    local.get 2
    i32.load offset=12
    i32.store
    local.get 0
    i32.const 44
    i32.add
    local.get 4
    i32.store
    local.get 0
    local.get 3
    i32.store offset=40
    local.get 0
    i64.const 1
    i64.store
    local.get 0
    local.get 1
    i64.load
    i64.store offset=8
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 0
    i32.const 32
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h292c2952bec8a29dE (type 7) (param i32) (result i32)
    local.get 0
    i64.const 4294967296
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h0af670e7d6bdc4d8E (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h2846b24ec5632223E (type 7) (param i32) (result i32)
    local.get 0
    i64.const 4294967296
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h5e1376fde4a99d56E (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h8816fe1a954ed65bE (type 7) (param i32) (result i32)
    local.get 0
    i64.const 2
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h868bc58131ae9d95E (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h93b0b6e2672b5543E (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E (type 4) (param i32 i32)
    (local i32 i32 i32 i64 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 24
    i32.add
    i32.const 4
    i32.or
    local.set 3
    i32.const 0
    local.set 4
    block  ;; label = @1
      loop  ;; label = @2
        block  ;; label = @3
          local.get 4
          i32.const 32
          i32.ne
          br_if 0 (;@3;)
          local.get 2
          i32.const 16
          i32.add
          i32.const 2
          i32.add
          local.tee 4
          local.get 2
          i32.const 37
          i32.add
          i32.load8_u
          i32.store8
          local.get 2
          i32.const 12
          i32.add
          i32.const 2
          i32.add
          local.tee 1
          local.get 2
          i32.const 44
          i32.add
          i32.load8_u
          i32.store8
          local.get 2
          i32.const 8
          i32.add
          i32.const 2
          i32.add
          local.tee 3
          local.get 2
          i32.const 51
          i32.add
          i32.load8_u
          i32.store8
          local.get 2
          local.get 2
          i32.load16_u offset=28
          i32.store16 offset=20
          local.get 2
          local.get 2
          i32.load8_u offset=30
          i32.store8 offset=22
          local.get 2
          local.get 2
          i32.const 24
          i32.add
          i32.const 11
          i32.add
          i32.load16_u align=1
          i32.store16 offset=16
          local.get 2
          local.get 2
          i32.const 24
          i32.add
          i32.const 18
          i32.add
          i32.load16_u
          i32.store16 offset=12
          local.get 2
          local.get 2
          i32.const 24
          i32.add
          i32.const 25
          i32.add
          i32.load16_u align=1
          i32.store16 offset=8
          local.get 2
          i32.const 52
          i32.add
          i64.load align=4
          local.set 5
          local.get 2
          i32.const 45
          i32.add
          i32.load align=1
          local.set 6
          local.get 2
          i32.const 38
          i32.add
          i32.load align=2
          local.set 7
          local.get 2
          i32.load offset=31 align=1
          local.set 8
          local.get 0
          i32.const 3
          i32.add
          local.get 2
          i32.load8_u offset=22
          i32.store8
          local.get 0
          local.get 2
          i32.load16_u offset=20
          i32.store16 offset=1 align=1
          local.get 0
          i32.const 4
          i32.add
          local.get 8
          i32.store align=1
          local.get 0
          i32.const 8
          i32.add
          local.get 2
          i32.load16_u offset=16
          i32.store16 align=1
          local.get 0
          i32.const 10
          i32.add
          local.get 4
          i32.load8_u
          i32.store8
          local.get 0
          i32.const 11
          i32.add
          local.get 7
          i32.store align=1
          local.get 0
          i32.const 15
          i32.add
          local.get 2
          i32.load16_u offset=12
          i32.store16 align=1
          local.get 0
          i32.const 17
          i32.add
          local.get 1
          i32.load8_u
          i32.store8
          local.get 0
          i32.const 18
          i32.add
          local.get 6
          i32.store align=1
          local.get 0
          i32.const 22
          i32.add
          local.get 2
          i32.load16_u offset=8
          i32.store16 align=1
          local.get 0
          i32.const 24
          i32.add
          local.get 3
          i32.load8_u
          i32.store8
          local.get 0
          i32.const 25
          i32.add
          local.get 5
          i64.store align=1
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        call $_ZN56_$LT$u8$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h374e211fca1804deE
        block  ;; label = @3
          local.get 2
          i32.load8_u
          i32.const 1
          i32.and
          br_if 0 (;@3;)
          local.get 3
          local.get 4
          i32.add
          local.get 2
          i32.load8_u offset=1
          i32.store8
          local.get 2
          local.get 4
          i32.const 1
          i32.add
          local.tee 4
          i32.store offset=24
          br 1 (;@2;)
        end
      end
      i32.const 1
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store8
    local.get 2
    i32.const 64
    i32.add
    global.set 0)
  (func $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E (type 4) (param i32 i32)
    (local i32 i64 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 144
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 16
    i32.add
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 16
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store
    local.get 2
    i32.const 16
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load
    i64.store
    local.get 2
    i64.const 0
    i64.store offset=48
    local.get 2
    local.get 1
    i64.load
    i64.store offset=16
    local.get 2
    i32.const 16
    i32.add
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h816f7eb6f73483c5E
    local.set 1
    block  ;; label = @1
      local.get 0
      i64.load offset=40
      local.tee 3
      i64.const 2
      i64.eq
      br_if 0 (;@1;)
      local.get 0
      i32.const 40
      i32.add
      local.tee 4
      i32.load8_u offset=24
      local.set 5
      local.get 4
      i32.const 1
      i32.store8 offset=24
      local.get 5
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 3
        i64.const 1
        i64.eq
        br_if 0 (;@2;)
        local.get 2
        i32.const 104
        i32.add
        i32.const 24
        i32.add
        local.get 1
        i32.const 24
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 104
        i32.add
        i32.const 16
        i32.add
        local.get 1
        i32.const 16
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 104
        i32.add
        i32.const 8
        i32.add
        local.get 1
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 2
        local.get 1
        i64.load
        i64.store offset=104
        local.get 2
        i64.const 1
        i64.store offset=136
        local.get 2
        i32.const 104
        i32.add
        call $_ZN7ink_env3api22clear_contract_storage17hdcbe1526eb8c26b6E
        br 1 (;@1;)
      end
      local.get 2
      i32.const 104
      i32.add
      i32.const 24
      i32.add
      local.get 1
      i32.const 24
      i32.add
      i64.load
      i64.store
      local.get 2
      i32.const 104
      i32.add
      i32.const 16
      i32.add
      local.get 1
      i32.const 16
      i32.add
      i64.load
      i64.store
      local.get 2
      i32.const 104
      i32.add
      i32.const 8
      i32.add
      local.get 1
      i32.const 8
      i32.add
      i64.load
      i64.store
      local.get 2
      local.get 1
      i64.load
      i64.store offset=104
      local.get 2
      i64.const 0
      i64.store offset=136
      local.get 2
      i32.const 104
      i32.add
      call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hcdf8ffb9d0b57905E
      local.set 1
      local.get 2
      i32.const 72
      i32.add
      i32.const 8
      i32.add
      i32.const 16384
      i32.store
      local.get 2
      i32.const 68528
      i32.store offset=76
      local.get 2
      i32.const 0
      i32.store offset=72
      local.get 2
      i32.const 8
      i32.add
      local.get 2
      i32.const 72
      i32.add
      local.get 4
      i32.const 8
      i32.add
      call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17hd2d0c7cb8dc086fdE
      local.get 1
      local.get 2
      i32.load offset=8
      local.get 2
      i32.load offset=12
      call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
    end
    local.get 0
    i32.const 72
    i32.add
    local.get 2
    i32.const 16
    i32.add
    call $_ZN94_$LT$ink_storage..pack..Pack$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11push_spread17h24aebc77a0eebb6eE
    local.get 2
    i32.const 16
    i32.add
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h292c2952bec8a29dE
    local.set 6
    local.get 0
    i32.const 176
    i32.add
    i32.load
    local.set 5
    local.get 0
    i32.const 168
    i32.add
    i32.load
    local.set 4
    local.get 2
    i32.const 104
    i32.add
    i32.const 24
    i32.add
    local.get 0
    i32.const 172
    i32.add
    i32.load
    local.tee 1
    i32.store
    local.get 2
    i32.const 124
    i32.add
    local.get 4
    i32.store
    local.get 2
    i32.const 104
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.store
    local.get 2
    local.get 4
    i32.store offset=108
    local.get 2
    local.get 1
    i32.eqz
    i32.const 1
    i32.shl
    local.tee 7
    i32.store offset=120
    local.get 2
    local.get 7
    i32.store offset=104
    local.get 5
    i32.const 0
    local.get 1
    select
    local.set 1
    local.get 2
    i32.const 104
    i32.add
    i32.const 4
    i32.or
    local.set 8
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          local.get 1
          i32.const -1
          i32.add
          i32.store offset=136
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              br_table 0 (;@5;) 1 (;@4;) 4 (;@1;) 1 (;@4;)
            end
            local.get 8
            local.get 2
            i32.load offset=108
            local.get 2
            i32.load offset=112
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h141cebf95d8eb32fE
            i32.const 1
            local.set 7
            local.get 2
            i32.const 1
            i32.store offset=104
          end
          local.get 2
          i32.load offset=116
          local.set 5
          local.get 2
          i32.load offset=112
          local.set 1
          local.get 2
          i32.load offset=108
          local.set 4
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              local.get 1
              i32.load16_u offset=94
              local.tee 9
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              i32.load
              local.tee 9
              i32.eqz
              br_if 4 (;@1;)
              local.get 4
              i32.const 1
              i32.add
              local.tee 10
              local.get 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              i32.load16_u offset=92
              local.set 5
              local.get 10
              local.set 4
              local.get 9
              local.set 1
              br 0 (;@5;)
            end
          end
          local.get 5
          local.get 9
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.eqz
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              br_if 0 (;@5;)
              local.get 5
              i32.const 1
              i32.add
              local.tee 4
              local.get 5
              i32.lt_u
              br_if 4 (;@1;)
              local.get 2
              local.get 4
              i32.store offset=80
              local.get 2
              local.get 1
              i32.store offset=76
              local.get 2
              i32.const 0
              i32.store offset=72
              br 1 (;@4;)
            end
            local.get 5
            i32.const 1
            i32.add
            local.tee 9
            local.get 5
            i32.lt_u
            br_if 3 (;@1;)
            local.get 4
            i32.const -1
            i32.add
            local.tee 10
            local.get 4
            i32.gt_u
            br_if 3 (;@1;)
            local.get 2
            i32.const 72
            i32.add
            local.get 10
            local.get 1
            local.get 9
            i32.const 2
            i32.shl
            i32.add
            i32.const 96
            i32.add
            i32.load
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h141cebf95d8eb32fE
          end
          local.get 8
          local.get 2
          i64.load offset=72
          i64.store align=4
          local.get 8
          i32.const 8
          i32.add
          local.get 2
          i32.const 72
          i32.add
          i32.const 8
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 72
          i32.add
          local.get 6
          local.get 1
          local.get 5
          i32.const 2
          i32.shl
          i32.add
          local.tee 1
          i32.const 4
          i32.add
          i64.load32_u
          call $_ZN81_$LT$$RF$ink_primitives..key..Key$u20$as$u20$core..ops..arith..Add$LT$u64$GT$$GT$3add17h00493616124916c0E
          local.get 1
          i32.const 48
          i32.add
          i32.load
          local.tee 1
          i32.load8_u offset=36
          local.set 4
          local.get 1
          i32.const 1
          i32.store8 offset=36
          block  ;; label = @4
            local.get 4
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 4
              i32.const 2
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              i32.const 72
              i32.add
              call $_ZN7ink_env3api22clear_contract_storage17hdcbe1526eb8c26b6E
              br 1 (;@4;)
            end
            local.get 2
            i32.const 16384
            i32.store offset=60
            local.get 2
            i32.const 68528
            i32.store offset=56
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.store8 offset=68528
                local.get 2
                i32.const 1
                i32.store offset=64
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.get 1
                i32.const 8
                i32.add
                i32.load
                local.get 2
                i32.const 56
                i32.add
                call $_ZN11ink_storage11collections5stash1_108_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_storage..collections..stash..VacantEntry$GT$9encode_to17h1d8d3bcccb1b1c07E
                br 1 (;@5;)
              end
              i32.const 0
              i32.const 1
              i32.store8 offset=68528
              local.get 2
              i32.const 1
              i32.store offset=64
              local.get 1
              i32.const 1
              i32.add
              local.get 2
              i32.const 56
              i32.add
              call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E
            end
            local.get 2
            i32.load offset=60
            local.get 2
            i32.load offset=64
            local.tee 1
            i32.lt_u
            br_if 3 (;@1;)
            local.get 2
            i32.const 72
            i32.add
            local.get 2
            i32.load offset=56
            local.get 1
            call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
          end
          local.get 2
          i32.load offset=136
          local.set 1
          br 0 (;@3;)
        end
      end
      local.get 2
      i32.const 16
      i32.add
      call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h0af670e7d6bdc4d8E
      local.set 6
      local.get 0
      i32.const 232
      i32.add
      i32.load
      local.set 5
      local.get 0
      i32.const 224
      i32.add
      i32.load
      local.set 4
      local.get 2
      i32.const 128
      i32.add
      local.get 0
      i32.const 228
      i32.add
      i32.load
      local.tee 1
      i32.store
      local.get 2
      i32.const 124
      i32.add
      local.get 4
      i32.store
      local.get 2
      i32.const 104
      i32.add
      i32.const 8
      i32.add
      local.get 1
      i32.store
      local.get 2
      local.get 4
      i32.store offset=108
      local.get 2
      local.get 1
      i32.eqz
      i32.const 1
      i32.shl
      local.tee 7
      i32.store offset=120
      local.get 2
      local.get 7
      i32.store offset=104
      local.get 5
      i32.const 0
      local.get 1
      select
      local.set 1
      local.get 2
      i32.const 104
      i32.add
      i32.const 4
      i32.or
      local.set 8
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          local.get 1
          i32.const -1
          i32.add
          i32.store offset=136
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              br_table 0 (;@5;) 1 (;@4;) 4 (;@1;) 1 (;@4;)
            end
            local.get 8
            local.get 2
            i32.load offset=108
            local.get 2
            i32.load offset=112
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h6d98a96bb7b5a593E
            i32.const 1
            local.set 7
            local.get 2
            i32.const 1
            i32.store offset=104
          end
          local.get 2
          i32.load offset=116
          local.set 5
          local.get 2
          i32.load offset=112
          local.set 1
          local.get 2
          i32.load offset=108
          local.set 4
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              local.get 1
              i32.load16_u offset=50
              local.tee 9
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              i32.load
              local.tee 9
              i32.eqz
              br_if 4 (;@1;)
              local.get 4
              i32.const 1
              i32.add
              local.tee 10
              local.get 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              i32.load16_u offset=48
              local.set 5
              local.get 10
              local.set 4
              local.get 9
              local.set 1
              br 0 (;@5;)
            end
          end
          local.get 5
          local.get 9
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.eqz
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              br_if 0 (;@5;)
              local.get 5
              i32.const 1
              i32.add
              local.tee 4
              local.get 5
              i32.lt_u
              br_if 4 (;@1;)
              local.get 2
              local.get 4
              i32.store offset=80
              local.get 2
              local.get 1
              i32.store offset=76
              local.get 2
              i32.const 0
              i32.store offset=72
              br 1 (;@4;)
            end
            local.get 5
            i32.const 1
            i32.add
            local.tee 9
            local.get 5
            i32.lt_u
            br_if 3 (;@1;)
            local.get 4
            i32.const -1
            i32.add
            local.tee 10
            local.get 4
            i32.gt_u
            br_if 3 (;@1;)
            local.get 2
            i32.const 72
            i32.add
            local.get 10
            local.get 1
            local.get 9
            i32.const 2
            i32.shl
            i32.add
            i32.const 404
            i32.add
            i32.load
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h6d98a96bb7b5a593E
          end
          local.get 8
          local.get 2
          i64.load offset=72
          i64.store align=4
          local.get 8
          i32.const 8
          i32.add
          local.get 2
          i32.const 72
          i32.add
          i32.const 8
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 72
          i32.add
          local.get 6
          local.get 1
          local.get 5
          i32.const 5
          i32.shl
          i32.add
          i32.const 52
          i32.add
          call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$13to_offset_key17h913ecdf8713f958cE
          local.get 1
          local.get 5
          i32.const 2
          i32.shl
          i32.add
          i32.const 4
          i32.add
          i32.load
          local.get 2
          i32.const 72
          i32.add
          call $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$16push_packed_root17h9fc891b41834526fE
          local.get 2
          i32.load offset=136
          local.set 1
          br 0 (;@3;)
        end
      end
      local.get 0
      i32.const 240
      i32.add
      local.get 2
      i32.const 16
      i32.add
      call $_ZN94_$LT$ink_storage..pack..Pack$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11push_spread17h24aebc77a0eebb6eE
      local.get 2
      i32.const 16
      i32.add
      call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h2846b24ec5632223E
      local.set 6
      local.get 0
      i32.const 344
      i32.add
      i32.load
      local.set 5
      local.get 0
      i32.const 336
      i32.add
      i32.load
      local.set 4
      local.get 2
      i32.const 128
      i32.add
      local.get 0
      i32.const 340
      i32.add
      i32.load
      local.tee 1
      i32.store
      local.get 2
      i32.const 124
      i32.add
      local.get 4
      i32.store
      local.get 2
      i32.const 104
      i32.add
      i32.const 8
      i32.add
      local.get 1
      i32.store
      local.get 2
      local.get 4
      i32.store offset=108
      local.get 2
      local.get 1
      i32.eqz
      i32.const 1
      i32.shl
      local.tee 7
      i32.store offset=120
      local.get 2
      local.get 7
      i32.store offset=104
      local.get 5
      i32.const 0
      local.get 1
      select
      local.set 1
      local.get 2
      i32.const 104
      i32.add
      i32.const 4
      i32.or
      local.set 8
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          local.get 1
          i32.const -1
          i32.add
          i32.store offset=136
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              br_table 0 (;@5;) 1 (;@4;) 4 (;@1;) 1 (;@4;)
            end
            local.get 8
            local.get 2
            i32.load offset=108
            local.get 2
            i32.load offset=112
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h416f8ce0597c47a5E
            i32.const 1
            local.set 7
            local.get 2
            i32.const 1
            i32.store offset=104
          end
          local.get 2
          i32.load offset=116
          local.set 5
          local.get 2
          i32.load offset=112
          local.set 1
          local.get 2
          i32.load offset=108
          local.set 4
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              local.get 1
              i32.load16_u offset=94
              local.tee 9
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              i32.load
              local.tee 9
              i32.eqz
              br_if 4 (;@1;)
              local.get 4
              i32.const 1
              i32.add
              local.tee 10
              local.get 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              i32.load16_u offset=92
              local.set 5
              local.get 10
              local.set 4
              local.get 9
              local.set 1
              br 0 (;@5;)
            end
          end
          local.get 5
          local.get 9
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.eqz
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              br_if 0 (;@5;)
              local.get 5
              i32.const 1
              i32.add
              local.tee 4
              local.get 5
              i32.lt_u
              br_if 4 (;@1;)
              local.get 2
              local.get 4
              i32.store offset=80
              local.get 2
              local.get 1
              i32.store offset=76
              local.get 2
              i32.const 0
              i32.store offset=72
              br 1 (;@4;)
            end
            local.get 5
            i32.const 1
            i32.add
            local.tee 9
            local.get 5
            i32.lt_u
            br_if 3 (;@1;)
            local.get 4
            i32.const -1
            i32.add
            local.tee 10
            local.get 4
            i32.gt_u
            br_if 3 (;@1;)
            local.get 2
            i32.const 72
            i32.add
            local.get 10
            local.get 1
            local.get 9
            i32.const 2
            i32.shl
            i32.add
            i32.const 96
            i32.add
            i32.load
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h416f8ce0597c47a5E
          end
          local.get 8
          local.get 2
          i64.load offset=72
          i64.store align=4
          local.get 8
          i32.const 8
          i32.add
          local.get 2
          i32.const 72
          i32.add
          i32.const 8
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 72
          i32.add
          local.get 6
          local.get 1
          local.get 5
          i32.const 2
          i32.shl
          i32.add
          local.tee 1
          i32.const 4
          i32.add
          i64.load32_u
          call $_ZN81_$LT$$RF$ink_primitives..key..Key$u20$as$u20$core..ops..arith..Add$LT$u64$GT$$GT$3add17h00493616124916c0E
          local.get 1
          i32.const 48
          i32.add
          i32.load
          local.tee 1
          i32.load8_u offset=68
          local.set 4
          local.get 1
          i32.const 1
          i32.store8 offset=68
          block  ;; label = @4
            local.get 4
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            block  ;; label = @5
              local.get 1
              i32.load8_u
              local.tee 4
              i32.const 2
              i32.ne
              br_if 0 (;@5;)
              local.get 2
              i32.const 72
              i32.add
              call $_ZN7ink_env3api22clear_contract_storage17hdcbe1526eb8c26b6E
              br 1 (;@4;)
            end
            local.get 2
            i32.const 16384
            i32.store offset=60
            local.get 2
            i32.const 68528
            i32.store offset=56
            block  ;; label = @5
              block  ;; label = @6
                local.get 4
                i32.const 1
                i32.eq
                br_if 0 (;@6;)
                i32.const 0
                i32.const 0
                i32.store8 offset=68528
                local.get 2
                i32.const 1
                i32.store offset=64
                local.get 1
                i32.const 4
                i32.add
                i32.load
                local.get 1
                i32.const 8
                i32.add
                i32.load
                local.get 2
                i32.const 56
                i32.add
                call $_ZN11ink_storage11collections5stash1_108_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_storage..collections..stash..VacantEntry$GT$9encode_to17h1d8d3bcccb1b1c07E
                br 1 (;@5;)
              end
              i32.const 0
              i32.const 1
              i32.store8 offset=68528
              local.get 2
              i32.const 1
              i32.store offset=64
              local.get 1
              i32.const 1
              i32.add
              local.get 2
              i32.const 56
              i32.add
              call $_ZN18parity_scale_codec5codec16inner_tuple_impl79_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$$LP$Q0$C$R0$RP$$GT$9encode_to17h88b7a0b7cfe1929aE
            end
            local.get 2
            i32.load offset=60
            local.get 2
            i32.load offset=64
            local.tee 1
            i32.lt_u
            br_if 3 (;@1;)
            local.get 2
            i32.const 72
            i32.add
            local.get 2
            i32.load offset=56
            local.get 1
            call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
          end
          local.get 2
          i32.load offset=136
          local.set 1
          br 0 (;@3;)
        end
      end
      local.get 2
      i32.const 16
      i32.add
      call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h5e1376fde4a99d56E
      local.set 6
      local.get 0
      i32.const 400
      i32.add
      i32.load
      local.set 5
      local.get 0
      i32.const 392
      i32.add
      i32.load
      local.set 4
      local.get 2
      i32.const 128
      i32.add
      local.get 0
      i32.const 396
      i32.add
      i32.load
      local.tee 1
      i32.store
      local.get 2
      i32.const 124
      i32.add
      local.get 4
      i32.store
      local.get 2
      i32.const 104
      i32.add
      i32.const 8
      i32.add
      local.get 1
      i32.store
      local.get 2
      local.get 4
      i32.store offset=108
      local.get 2
      local.get 1
      i32.eqz
      i32.const 1
      i32.shl
      local.tee 7
      i32.store offset=120
      local.get 2
      local.get 7
      i32.store offset=104
      local.get 5
      i32.const 0
      local.get 1
      select
      local.set 1
      local.get 2
      i32.const 104
      i32.add
      i32.const 4
      i32.or
      local.set 8
      block  ;; label = @2
        loop  ;; label = @3
          local.get 1
          i32.eqz
          br_if 1 (;@2;)
          local.get 2
          local.get 1
          i32.const -1
          i32.add
          i32.store offset=136
          block  ;; label = @4
            block  ;; label = @5
              local.get 7
              br_table 0 (;@5;) 1 (;@4;) 4 (;@1;) 1 (;@4;)
            end
            local.get 8
            local.get 2
            i32.load offset=108
            local.get 2
            i32.load offset=112
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h86a928d7d9c4e05aE
            i32.const 1
            local.set 7
            local.get 2
            i32.const 1
            i32.store offset=104
          end
          local.get 2
          i32.load offset=116
          local.set 5
          local.get 2
          i32.load offset=112
          local.set 1
          local.get 2
          i32.load offset=108
          local.set 4
          block  ;; label = @4
            loop  ;; label = @5
              local.get 5
              local.get 1
              i32.load16_u offset=50
              local.tee 9
              i32.lt_u
              br_if 1 (;@4;)
              local.get 1
              i32.load
              local.tee 9
              i32.eqz
              br_if 4 (;@1;)
              local.get 4
              i32.const 1
              i32.add
              local.tee 10
              local.get 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              i32.load16_u offset=48
              local.set 5
              local.get 10
              local.set 4
              local.get 9
              local.set 1
              br 0 (;@5;)
            end
          end
          local.get 5
          local.get 9
          i32.ge_u
          br_if 2 (;@1;)
          local.get 1
          i32.eqz
          br_if 2 (;@1;)
          block  ;; label = @4
            block  ;; label = @5
              local.get 4
              br_if 0 (;@5;)
              local.get 5
              i32.const 1
              i32.add
              local.tee 4
              local.get 5
              i32.lt_u
              br_if 4 (;@1;)
              local.get 2
              local.get 4
              i32.store offset=80
              local.get 2
              local.get 1
              i32.store offset=76
              local.get 2
              i32.const 0
              i32.store offset=72
              br 1 (;@4;)
            end
            local.get 5
            i32.const 1
            i32.add
            local.tee 9
            local.get 5
            i32.lt_u
            br_if 3 (;@1;)
            local.get 4
            i32.const -1
            i32.add
            local.tee 10
            local.get 4
            i32.gt_u
            br_if 3 (;@1;)
            local.get 2
            i32.const 72
            i32.add
            local.get 10
            local.get 1
            local.get 9
            i32.const 2
            i32.shl
            i32.add
            i32.const 756
            i32.add
            i32.load
            call $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h86a928d7d9c4e05aE
          end
          local.get 8
          local.get 2
          i64.load offset=72
          i64.store align=4
          local.get 8
          i32.const 8
          i32.add
          local.get 2
          i32.const 72
          i32.add
          i32.const 8
          i32.add
          i32.load
          i32.store
          local.get 2
          i32.const 72
          i32.add
          local.get 6
          local.get 1
          local.get 5
          i32.const 6
          i32.shl
          i32.add
          i32.const 52
          i32.add
          call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$13to_offset_key17h7fe2167c13a89784E
          local.get 1
          local.get 5
          i32.const 2
          i32.shl
          i32.add
          i32.const 4
          i32.add
          i32.load
          local.get 2
          i32.const 72
          i32.add
          call $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$16push_packed_root17h9fc891b41834526fE
          local.get 2
          i32.load offset=136
          local.set 1
          br 0 (;@3;)
        end
      end
      local.get 0
      i32.const 408
      i32.add
      local.get 2
      i32.const 16
      i32.add
      call $_ZN109_$LT$ink_storage..lazy..lazy_cell..LazyCell$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11push_spread17hfda51ce49dd0a410E
      local.get 0
      i32.const 472
      i32.add
      local.get 2
      i32.const 16
      i32.add
      call $_ZN109_$LT$ink_storage..lazy..lazy_cell..LazyCell$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11push_spread17hfda51ce49dd0a410E
      local.get 2
      i32.const 16
      i32.add
      call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h868bc58131ae9d95E
      local.set 4
      block  ;; label = @2
        local.get 0
        i32.const 576
        i32.add
        local.tee 1
        i32.load8_u
        local.tee 5
        i32.const 2
        i32.eq
        br_if 0 (;@2;)
        local.get 1
        i32.load8_u offset=2
        local.set 9
        local.get 1
        i32.const 1
        i32.store8 offset=2
        local.get 9
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 5
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.const 104
          i32.add
          i32.const 24
          i32.add
          local.get 4
          i32.const 24
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 104
          i32.add
          i32.const 16
          i32.add
          local.get 4
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 104
          i32.add
          i32.const 8
          i32.add
          local.get 4
          i32.const 8
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 4
          i64.load
          i64.store offset=104
          local.get 2
          i64.const 1
          i64.store offset=136
          local.get 2
          i32.const 104
          i32.add
          call $_ZN7ink_env3api22clear_contract_storage17hdcbe1526eb8c26b6E
          br 1 (;@2;)
        end
        local.get 2
        i32.const 104
        i32.add
        i32.const 24
        i32.add
        local.get 4
        i32.const 24
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 104
        i32.add
        i32.const 16
        i32.add
        local.get 4
        i32.const 16
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 104
        i32.add
        i32.const 8
        i32.add
        local.get 4
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 2
        local.get 4
        i64.load
        i64.store offset=104
        local.get 2
        i64.const 0
        i64.store offset=136
        local.get 1
        i32.const 1
        i32.add
        local.get 2
        i32.const 104
        i32.add
        call $_ZN11ink_storage6traits5impls5prims1_74_$LT$impl$u20$ink_storage..traits..spread..SpreadLayout$u20$for$u20$u8$GT$11push_spread17h14d5413c06275cf2E
      end
      local.get 2
      i32.const 16
      i32.add
      call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h93b0b6e2672b5543E
      local.set 1
      local.get 2
      i32.const 112
      i32.add
      i32.const 16384
      i32.store
      local.get 2
      i32.const 68528
      i32.store offset=108
      local.get 2
      i32.const 0
      i32.store offset=104
      local.get 2
      local.get 2
      i32.const 104
      i32.add
      local.get 0
      i32.const 584
      i32.add
      call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h57c9f8b1d4f26e96E
      local.get 1
      local.get 2
      i32.load
      local.get 2
      i32.load offset=4
      call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
      local.get 2
      i32.const 144
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN7ink_env3api22clear_contract_storage17hdcbe1526eb8c26b6E (type 5) (param i32)
    local.get 0
    call $_ZN7ink_env6engine8on_chain3ext3sys18seal_clear_storage17ha0e0a8a3baefb6bcE)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hcdf8ffb9d0b57905E (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17hd2d0c7cb8dc086fdE (type 2) (param i32 i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i64.load offset=4 align=4
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    local.get 4
    i64.store offset=16
    local.get 2
    i64.load
    local.get 2
    i32.const 8
    i32.add
    i64.load
    local.get 3
    i32.const 16
    i32.add
    call $_ZN18parity_scale_codec5codec6Encode9encode_to17hc5f3c4e04a903aa0E
    local.get 1
    local.get 3
    i64.load offset=16
    i64.store offset=4 align=4
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 3
    i32.load offset=24
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer4take17hd78259512cf0470aE
    local.get 0
    local.get 3
    i64.load offset=8
    i64.store
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func $_ZN94_$LT$ink_storage..pack..Pack$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11push_spread17h24aebc77a0eebb6eE (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17ha548bed9e797b95bE
    local.set 1
    local.get 2
    i64.const 16384
    i64.store offset=4 align=4
    local.get 2
    i32.const 68528
    i32.store
    local.get 0
    i32.load offset=40
    local.get 2
    call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E
    local.get 0
    i32.const 44
    i32.add
    i32.load
    local.get 2
    call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E
    local.get 0
    i32.const 48
    i32.add
    i32.load
    local.get 2
    call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E
    block  ;; label = @1
      local.get 2
      i32.load offset=4
      local.get 2
      i32.load offset=8
      local.tee 0
      i32.ge_u
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 1
    local.get 2
    i32.load
    local.get 0
    call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h141cebf95d8eb32fE (type 2) (param i32 i32 i32)
    loop  ;; label = @1
      block  ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      local.get 1
      i32.const -1
      i32.add
      local.set 1
      local.get 2
      i32.load offset=96
      local.set 2
      br 0 (;@1;)
    end)
  (func $_ZN81_$LT$$RF$ink_primitives..key..Key$u20$as$u20$core..ops..arith..Add$LT$u64$GT$$GT$3add17h00493616124916c0E (type 9) (param i32 i32 i64)
    (local i64 i64)
    local.get 0
    local.get 1
    i64.load
    local.tee 3
    local.get 2
    i64.add
    local.tee 2
    i64.store
    local.get 0
    local.get 1
    i64.load offset=8
    local.tee 4
    local.get 2
    local.get 3
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 2
    i64.store offset=8
    local.get 0
    local.get 1
    i64.load offset=16
    local.tee 3
    local.get 2
    local.get 4
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 2
    i64.store offset=16
    local.get 0
    local.get 1
    i64.load offset=24
    local.get 2
    local.get 3
    i64.lt_u
    i64.extend_i32_u
    i64.add
    i64.store offset=24)
  (func $_ZN11ink_storage11collections5stash1_108_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_storage..collections..stash..VacantEntry$GT$9encode_to17h1d8d3bcccb1b1c07E (type 2) (param i32 i32 i32)
    local.get 0
    local.get 2
    call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E
    local.get 1
    local.get 2
    call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E)
  (func $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E (type 4) (param i32 i32)
    local.get 1
    local.get 0
    i32.const 32
    call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E)
  (func $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h6d98a96bb7b5a593E (type 2) (param i32 i32 i32)
    loop  ;; label = @1
      block  ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      local.get 1
      i32.const -1
      i32.add
      local.set 1
      local.get 2
      i32.load offset=404
      local.set 2
      br 0 (;@1;)
    end)
  (func $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$13to_offset_key17h913ecdf8713f958cE (type 2) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 26
    i32.add
    i32.const 112
    i32.store8
    local.get 3
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    i32.const 24941
    i32.store16
    local.get 3
    i64.const 7526466502114635369
    i64.store offset=16
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    i32.const 32
    i32.add
    i32.const 24
    i32.add
    local.tee 1
    i64.const 0
    i64.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 16
    i32.add
    local.tee 4
    i64.const 0
    i64.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    local.tee 5
    i64.const 0
    i64.store
    local.get 3
    i64.const 0
    i64.store offset=32
    local.get 3
    i64.const 16384
    i64.store offset=68 align=4
    local.get 3
    i32.const 68528
    i32.store offset=64
    local.get 3
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 3
    i32.const 64
    i32.add
    call $_ZN76_$LT$$u5b$T$u3b$$u20$N$u5d$$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h01fc34d3619bb365E
    local.get 3
    i32.const 8
    i32.add
    local.get 3
    i32.const 64
    i32.add
    call $_ZN55_$LT$X$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h50e64f9885339a4cE
    local.get 2
    local.get 3
    i32.const 64
    i32.add
    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E
    block  ;; label = @1
      local.get 3
      i32.load offset=68
      local.get 3
      i32.load offset=72
      local.tee 2
      i32.ge_u
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 3
    i32.load offset=64
    local.get 2
    local.get 3
    i32.const 32
    i32.add
    call $_ZN7ink_env6engine8on_chain3ext3sys20seal_hash_blake2_25617h93084b8f47746e4fE
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i64.load
    i64.store align=1
    local.get 0
    i32.const 16
    i32.add
    local.get 4
    i64.load
    i64.store align=1
    local.get 0
    i32.const 8
    i32.add
    local.get 5
    i64.load
    i64.store align=1
    local.get 0
    local.get 3
    i64.load offset=32
    i64.store align=1
    local.get 3
    i32.const 80
    i32.add
    global.set 0)
  (func $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$16push_packed_root17h9fc891b41834526fE (type 4) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load8_u offset=32
    local.set 3
    local.get 0
    i32.const 1
    i32.store8 offset=32
    block  ;; label = @1
      block  ;; label = @2
        local.get 3
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 0
          i64.load
          i64.const 1
          i64.eq
          br_if 0 (;@3;)
          local.get 1
          call $_ZN7ink_env3api22clear_contract_storage17hdcbe1526eb8c26b6E
          br 1 (;@2;)
        end
        local.get 2
        i64.const 16384
        i64.store offset=4 align=4
        local.get 2
        i32.const 68528
        i32.store
        local.get 0
        i64.load offset=8
        local.get 0
        i32.const 16
        i32.add
        i64.load
        local.get 2
        call $_ZN18parity_scale_codec5codec6Encode9encode_to17hc5f3c4e04a903aa0E
        local.get 0
        i32.const 24
        i32.add
        i32.load
        local.get 2
        call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E
        local.get 2
        i32.load offset=4
        local.get 2
        i32.load offset=8
        local.tee 0
        i32.lt_u
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.load
        local.get 0
        call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
      end
      local.get 2
      i32.const 16
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h416f8ce0597c47a5E (type 2) (param i32 i32 i32)
    loop  ;; label = @1
      block  ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      local.get 1
      i32.const -1
      i32.add
      local.set 1
      local.get 2
      i32.load offset=96
      local.set 2
      br 0 (;@1;)
    end)
  (func $_ZN18parity_scale_codec5codec16inner_tuple_impl79_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$$LP$Q0$C$R0$RP$$GT$9encode_to17h88b7a0b7cfe1929aE (type 4) (param i32 i32)
    local.get 0
    local.get 1
    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E
    local.get 0
    i32.const 32
    i32.add
    local.get 1
    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E)
  (func $_ZN5alloc11collections5btree8navigate142_$LT$impl$u20$alloc..collections..btree..node..NodeRef$LT$BorrowType$C$K$C$V$C$alloc..collections..btree..node..marker..LeafOrInternal$GT$$GT$15first_leaf_edge17h86a928d7d9c4e05aE (type 2) (param i32 i32 i32)
    loop  ;; label = @1
      block  ;; label = @2
        local.get 1
        br_if 0 (;@2;)
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 2
        i32.store offset=4
        local.get 0
        i32.const 0
        i32.store
        return
      end
      local.get 1
      i32.const -1
      i32.add
      local.set 1
      local.get 2
      i32.load offset=756
      local.set 2
      br 0 (;@1;)
    end)
  (func $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$13to_offset_key17h7fe2167c13a89784E (type 2) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 26
    i32.add
    i32.const 112
    i32.store8
    local.get 3
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    i32.const 24941
    i32.store16
    local.get 3
    i64.const 7526466502114635369
    i64.store offset=16
    local.get 3
    local.get 2
    i32.store offset=12
    local.get 3
    local.get 1
    i32.store offset=8
    local.get 3
    i32.const 32
    i32.add
    i32.const 24
    i32.add
    local.tee 1
    i64.const 0
    i64.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 16
    i32.add
    local.tee 4
    i64.const 0
    i64.store
    local.get 3
    i32.const 32
    i32.add
    i32.const 8
    i32.add
    local.tee 5
    i64.const 0
    i64.store
    local.get 3
    i64.const 0
    i64.store offset=32
    local.get 3
    i64.const 16384
    i64.store offset=68 align=4
    local.get 3
    i32.const 68528
    i32.store offset=64
    local.get 3
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 3
    i32.const 64
    i32.add
    call $_ZN76_$LT$$u5b$T$u3b$$u20$N$u5d$$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h01fc34d3619bb365E
    local.get 3
    i32.const 8
    i32.add
    local.get 3
    i32.const 64
    i32.add
    call $_ZN55_$LT$X$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h50e64f9885339a4cE
    local.get 2
    local.get 3
    i32.const 64
    i32.add
    call $_ZN18parity_scale_codec5codec16inner_tuple_impl79_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$$LP$Q0$C$R0$RP$$GT$9encode_to17h88b7a0b7cfe1929aE
    block  ;; label = @1
      local.get 3
      i32.load offset=68
      local.get 3
      i32.load offset=72
      local.tee 2
      i32.ge_u
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 3
    i32.load offset=64
    local.get 2
    local.get 3
    i32.const 32
    i32.add
    call $_ZN7ink_env6engine8on_chain3ext3sys20seal_hash_blake2_25617h93084b8f47746e4fE
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i64.load
    i64.store align=1
    local.get 0
    i32.const 16
    i32.add
    local.get 4
    i64.load
    i64.store align=1
    local.get 0
    i32.const 8
    i32.add
    local.get 5
    i64.load
    i64.store align=1
    local.get 0
    local.get 3
    i64.load offset=32
    i64.store align=1
    local.get 3
    i32.const 80
    i32.add
    global.set 0)
  (func $_ZN109_$LT$ink_storage..lazy..lazy_cell..LazyCell$LT$T$GT$$u20$as$u20$ink_storage..traits..spread..SpreadLayout$GT$11push_spread17hfda51ce49dd0a410E (type 4) (param i32 i32)
    (local i32 i32 i32 i64 i64 i64 i64)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17h8816fe1a954ed65bE
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=40
        local.tee 3
        i32.const 2
        i32.eq
        br_if 0 (;@2;)
        local.get 0
        i32.const 40
        i32.add
        local.tee 0
        i32.load8_u offset=16
        local.set 4
        local.get 0
        i32.const 1
        i32.store8 offset=16
        local.get 4
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 3
          i32.const 1
          i32.eq
          br_if 0 (;@3;)
          local.get 2
          i32.const 8
          i32.add
          i32.const 24
          i32.add
          local.get 1
          i32.const 24
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 8
          i32.add
          i32.const 16
          i32.add
          local.get 1
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 8
          i32.add
          i32.const 8
          i32.add
          local.get 1
          i32.const 8
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 1
          i64.load
          i64.store offset=8
          local.get 2
          i64.const 0
          i64.store offset=40
          i64.const 2
          local.set 5
          loop  ;; label = @4
            local.get 5
            i64.eqz
            br_if 2 (;@2;)
            local.get 2
            i64.load offset=40
            local.set 6
            local.get 2
            i64.const 1
            i64.store offset=40
            local.get 2
            local.get 6
            local.get 2
            i64.load offset=8
            local.tee 7
            i64.add
            local.tee 6
            i64.store offset=8
            local.get 2
            local.get 2
            i64.load offset=16
            local.tee 8
            local.get 6
            local.get 7
            i64.lt_u
            i64.extend_i32_u
            i64.add
            local.tee 6
            i64.store offset=16
            local.get 2
            local.get 2
            i64.load offset=24
            local.tee 7
            local.get 6
            local.get 8
            i64.lt_u
            i64.extend_i32_u
            i64.add
            local.tee 6
            i64.store offset=24
            local.get 2
            local.get 2
            i64.load offset=32
            local.get 6
            local.get 7
            i64.lt_u
            i64.extend_i32_u
            i64.add
            i64.store offset=32
            local.get 5
            i64.const -1
            i64.add
            local.set 5
            local.get 2
            i32.const 8
            i32.add
            call $_ZN7ink_env3api22clear_contract_storage17hdcbe1526eb8c26b6E
            br 0 (;@4;)
          end
        end
        local.get 2
        i32.const 8
        i32.add
        i32.const 24
        i32.add
        local.get 1
        i32.const 24
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 8
        i32.add
        i32.const 16
        i32.add
        local.get 1
        i32.const 16
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 8
        i32.add
        i32.const 8
        i32.add
        local.get 1
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 2
        local.get 1
        i64.load
        i64.store offset=8
        local.get 2
        i64.const 0
        i64.store offset=40
        local.get 2
        local.get 0
        i32.load offset=4
        local.tee 1
        i32.const 0
        i32.ne
        i32.store8 offset=48
        local.get 2
        i32.const 48
        i32.add
        local.get 2
        i32.const 8
        i32.add
        call $_ZN11ink_storage6traits5impls5prims1_74_$LT$impl$u20$ink_storage..traits..spread..SpreadLayout$u20$for$u20$u8$GT$11push_spread17h14d5413c06275cf2E
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        local.get 2
        i32.const 8
        i32.add
        call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hf1aaf24072cf789cE
        local.set 1
        local.get 2
        i64.const 16384
        i64.store offset=52 align=4
        local.get 2
        i32.const 68528
        i32.store offset=48
        local.get 0
        i32.const 4
        i32.add
        local.get 2
        i32.const 48
        i32.add
        call $_ZN55_$LT$X$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h8e4e3faea8e8d6f3E
        local.get 2
        i32.load offset=52
        local.get 2
        i32.load offset=56
        local.tee 0
        i32.lt_u
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.load offset=48
        local.get 0
        call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
      end
      local.get 2
      i32.const 64
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN11ink_storage6traits5impls5prims1_74_$LT$impl$u20$ink_storage..traits..spread..SpreadLayout$u20$for$u20$u8$GT$11push_spread17h14d5413c06275cf2E (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hfb57d7af5e666bbcE
    local.set 1
    local.get 2
    i32.const 24
    i32.add
    i32.const 16384
    i32.store
    local.get 2
    i32.const 68528
    i32.store offset=20
    local.get 2
    i32.const 0
    i32.store offset=16
    local.get 2
    i32.const 8
    i32.add
    local.get 2
    i32.const 16
    i32.add
    local.get 0
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h20963af99cf5deb7E
    local.get 1
    local.get 2
    i32.load offset=8
    local.get 2
    i32.load offset=12
    call $_ZN7ink_env6engine8on_chain3ext3sys16seal_set_storage17h61f1acb1f9457e41E
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h57c9f8b1d4f26e96E (type 2) (param i32 i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i64.load offset=4 align=4
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    local.get 4
    i64.store offset=16
    local.get 2
    local.get 3
    i32.const 16
    i32.add
    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E
    local.get 1
    local.get 3
    i64.load offset=16
    i64.store offset=4 align=4
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 3
    i32.load offset=24
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer4take17hd78259512cf0470aE
    local.get 0
    local.get 3
    i64.load offset=8
    i64.store
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3d900fd5aef00106E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    local.get 2
    local.get 1
    i32.load offset=24
    i32.const 68456
    i32.const 16
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 0)
    i32.store8 offset=8
    local.get 2
    local.get 1
    i32.store
    local.get 2
    i32.const 0
    i32.store8 offset=9
    local.get 2
    i32.const 0
    i32.store offset=4
    local.get 2
    local.get 0
    i32.store offset=12
    local.get 2
    local.get 2
    i32.const 12
    i32.add
    i32.const 68440
    call $_ZN4core3fmt8builders10DebugTuple5field17hfbb7b61041be766dE
    call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt8builders10DebugTuple5field17hfbb7b61041be766dE (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=8
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=4
        local.set 4
        i32.const 1
        local.set 5
        br 1 (;@1;)
      end
      local.get 0
      i32.load offset=4
      local.set 4
      block  ;; label = @2
        local.get 0
        i32.load
        local.tee 6
        i32.load
        local.tee 7
        i32.const 4
        i32.and
        br_if 0 (;@2;)
        i32.const 1
        local.set 5
        local.get 6
        i32.load offset=24
        i32.const 65790
        i32.const 65794
        local.get 4
        select
        i32.const 2
        i32.const 1
        local.get 4
        select
        local.get 6
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        br_if 1 (;@1;)
        local.get 1
        local.get 6
        local.get 2
        i32.load offset=12
        call_indirect (type 1)
        local.set 5
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 4
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 6
          i32.load offset=24
          i32.const 65792
          i32.const 2
          local.get 6
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          i32.eqz
          br_if 0 (;@3;)
          i32.const 1
          local.set 5
          i32.const 0
          local.set 4
          br 2 (;@1;)
        end
        local.get 6
        i32.load
        local.set 7
      end
      i32.const 1
      local.set 5
      local.get 3
      i32.const 1
      i32.store8 offset=23
      local.get 3
      i32.const 52
      i32.add
      i32.const 65760
      i32.store
      local.get 3
      i32.const 16
      i32.add
      local.get 3
      i32.const 23
      i32.add
      i32.store
      local.get 3
      local.get 7
      i32.store offset=24
      local.get 3
      local.get 6
      i64.load offset=24 align=4
      i64.store offset=8
      local.get 6
      i64.load offset=8 align=4
      local.set 8
      local.get 6
      i64.load offset=16 align=4
      local.set 9
      local.get 3
      local.get 6
      i32.load8_u offset=32
      i32.store8 offset=56
      local.get 3
      local.get 6
      i32.load offset=4
      i32.store offset=28
      local.get 3
      local.get 9
      i64.store offset=40
      local.get 3
      local.get 8
      i64.store offset=32
      local.get 3
      local.get 3
      i32.const 8
      i32.add
      i32.store offset=48
      local.get 1
      local.get 3
      i32.const 24
      i32.add
      local.get 2
      i32.load offset=12
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 3
      i32.load offset=48
      i32.const 65788
      i32.const 2
      local.get 3
      i32.load offset=52
      i32.load offset=12
      call_indirect (type 0)
      local.set 5
    end
    local.get 0
    local.get 5
    i32.store8 offset=8
    block  ;; label = @1
      local.get 4
      i32.const 1
      i32.add
      local.tee 5
      local.get 4
      i32.lt_u
      br_if 0 (;@1;)
      local.get 0
      local.get 5
      i32.store offset=4
      local.get 3
      i32.const 64
      i32.add
      global.set 0
      local.get 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE (type 7) (param i32) (result i32)
    (local i32 i32 i32)
    local.get 0
    i32.load8_u offset=8
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 1
      i32.const 255
      i32.and
      local.set 3
      i32.const 1
      local.set 1
      block  ;; label = @2
        local.get 3
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 2
          i32.const 1
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          i32.load8_u offset=9
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          i32.load
          local.tee 3
          i32.load8_u
          i32.const 4
          i32.and
          br_if 0 (;@3;)
          i32.const 1
          local.set 1
          local.get 3
          i32.load offset=24
          i32.const 65795
          i32.const 1
          local.get 3
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          br_if 1 (;@2;)
        end
        local.get 0
        i32.load
        local.tee 1
        i32.load offset=24
        i32.const 65796
        i32.const 1
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        local.set 1
      end
      local.get 0
      local.get 1
      i32.store8 offset=8
    end
    local.get 1
    i32.const 255
    i32.and
    i32.const 0
    i32.ne)
  (func $_ZN4core3ptr33drop_in_place$LT$$RF$$LP$$RP$$GT$17h79d37317ba0fbc4fE (type 5) (param i32))
  (func $_ZN4core3ptr58drop_in_place$LT$$RF$psp22..traits..PSP22ReceiverError$GT$17he5a41f53852e4afeE (type 5) (param i32))
  (func $_ZN87_$LT$ink_allocator..bump..BumpAllocator$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h3b5b87aeed817ef7E (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 1
        i32.add
        i32.const -1
        i32.add
        i32.const 0
        local.get 1
        i32.sub
        local.tee 2
        i32.and
        local.tee 3
        local.get 0
        i32.lt_u
        br_if 0 (;@2;)
        local.get 1
        i32.popcnt
        i32.const 1
        i32.ne
        br_if 1 (;@1;)
        local.get 3
        local.get 2
        i32.gt_u
        br_if 1 (;@1;)
        i32.const 0
        local.set 1
        block  ;; label = @3
          i32.const 0
          i32.load offset=68520
          local.tee 0
          local.get 3
          i32.add
          local.tee 2
          local.get 0
          i32.lt_u
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            i32.const 0
            i32.load offset=68524
            i32.le_u
            br_if 0 (;@4;)
            local.get 3
            i32.const 65536
            i32.add
            local.tee 0
            local.get 3
            i32.lt_u
            br_if 2 (;@2;)
            i32.const 0
            local.set 1
            local.get 0
            i32.const -1
            i32.add
            local.tee 2
            i32.const 16
            i32.shr_u
            memory.grow
            local.tee 0
            i32.const -1
            i32.eq
            br_if 1 (;@3;)
            local.get 0
            i32.const 65535
            i32.and
            local.get 0
            i32.ne
            br_if 1 (;@3;)
            local.get 0
            i32.const 16
            i32.shl
            local.tee 0
            local.get 2
            i32.const -65536
            i32.and
            i32.add
            local.tee 2
            local.get 0
            i32.lt_u
            br_if 1 (;@3;)
            i32.const 0
            local.set 1
            i32.const 0
            local.get 2
            i32.store offset=68524
            local.get 0
            local.get 3
            i32.add
            local.tee 2
            local.get 0
            i32.lt_u
            br_if 1 (;@3;)
          end
          i32.const 0
          local.get 2
          i32.store offset=68520
          local.get 0
          local.set 1
        end
        local.get 1
        return
      end
      unreachable
      unreachable
    end
    call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
    unreachable)
  (func $_ZN5psp226traits5PSP2210balance_of17h8e8fb26b27704ba9E (type 2) (param i32 i32 i32)
    (local i64 i64)
    i64.const 0
    local.set 3
    i64.const 0
    local.set 4
    block  ;; label = @1
      local.get 1
      i32.const 184
      i32.add
      local.get 2
      call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$11lazily_load17hcfed61e18df0657dE
      local.tee 2
      i64.load
      i64.const 1
      i64.ne
      br_if 0 (;@1;)
      local.get 2
      i32.const 16
      i32.add
      i64.load
      local.set 4
      local.get 2
      i64.load offset=8
      local.set 3
    end
    local.get 0
    local.get 3
    i64.store
    local.get 0
    local.get 4
    i64.store offset=8)
  (func $_ZN5psp226traits5PSP2212total_supply17h3afc31b33677089bE (type 4) (param i32 i32)
    (local i32 i64 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 112
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i64.load offset=40
        local.tee 3
        i64.const 2
        i64.ne
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i64.load
            i64.const 1
            i64.ne
            br_if 0 (;@4;)
            local.get 2
            i32.const 24
            i32.add
            i32.const 24
            i32.add
            local.tee 4
            local.get 1
            i32.const 32
            i32.add
            i64.load
            i64.store
            local.get 2
            i32.const 24
            i32.add
            i32.const 16
            i32.add
            local.tee 5
            local.get 1
            i32.const 24
            i32.add
            i64.load
            i64.store
            local.get 2
            i32.const 24
            i32.add
            i32.const 8
            i32.add
            local.tee 6
            local.get 1
            i32.const 16
            i32.add
            i64.load
            i64.store
            local.get 2
            local.get 1
            i64.load offset=8
            i64.store offset=24
            local.get 2
            i32.const 24
            i32.add
            call $_ZN7ink_env3api20get_contract_storage17hed1fdc7f0b1449efE
            i32.const 1
            i32.and
            i32.eqz
            br_if 0 (;@4;)
            local.get 2
            i32.const 56
            i32.add
            i32.const 24
            i32.add
            local.get 4
            i64.load
            i64.store
            local.get 2
            i32.const 56
            i32.add
            i32.const 16
            i32.add
            local.get 5
            i64.load
            i64.store
            local.get 2
            i32.const 56
            i32.add
            i32.const 8
            i32.add
            local.get 6
            i64.load
            i64.store
            local.get 2
            local.get 2
            i64.load offset=24
            i64.store offset=56
            local.get 2
            i64.const 0
            i64.store offset=88
            local.get 2
            i32.const 56
            i32.add
            call $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hcdf8ffb9d0b57905E
            local.set 4
            local.get 2
            i32.const 16384
            i32.store offset=100
            local.get 2
            i32.const 68528
            i32.store offset=96
            block  ;; label = @5
              local.get 4
              local.get 2
              i32.const 96
              i32.add
              call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
              local.tee 4
              i32.const 13
              i32.eq
              br_if 0 (;@5;)
              local.get 4
              i32.const 3
              i32.eq
              br_if 4 (;@1;)
              unreachable
              unreachable
            end
            local.get 2
            local.get 2
            i64.load offset=96
            i64.store offset=104
            local.get 2
            local.get 2
            i32.const 104
            i32.add
            call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
            block  ;; label = @5
              local.get 2
              i64.load
              i32.wrap_i64
              br_if 0 (;@5;)
              local.get 2
              i32.const 16
              i32.add
              i64.load
              local.set 7
              local.get 2
              i64.load offset=8
              local.set 8
              i64.const 1
              local.set 3
              br 2 (;@3;)
            end
            call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
            unreachable
          end
          i64.const 0
          local.set 3
        end
        local.get 1
        local.get 3
        i64.store offset=40
        local.get 1
        i32.const 56
        i32.add
        local.get 7
        i64.store
        local.get 1
        i32.const 48
        i32.add
        local.get 8
        i64.store
        local.get 1
        i32.const 64
        i32.add
        i32.const 1
        i32.store8
      end
      local.get 3
      i64.const 1
      i64.ne
      br_if 0 (;@1;)
      local.get 0
      local.get 1
      i64.load offset=48
      i64.store
      local.get 0
      local.get 1
      i32.const 56
      i32.add
      i64.load
      i64.store offset=8
      local.get 2
      i32.const 112
      i32.add
      global.set 0
      return
    end
    call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
    unreachable)
  (func $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E (type 4) (param i32 i32)
    (local i32 i64 i32 i64 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    i64.const 0
    local.set 3
    local.get 2
    i32.const 8
    i32.add
    local.tee 4
    i64.const 0
    i64.store
    local.get 2
    i64.const 0
    i64.store
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.const 16
        call $_ZN69_$LT$$RF$$u5b$u8$u5d$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17ha43a819276410a89E
        br_if 0 (;@2;)
        local.get 4
        i64.load
        local.set 5
        local.get 2
        i64.load
        local.set 6
        br 1 (;@1;)
      end
      i64.const 1
      local.set 3
    end
    local.get 0
    local.get 6
    i64.store offset=8
    local.get 0
    local.get 3
    i64.store
    local.get 0
    i32.const 16
    i32.add
    local.get 5
    i64.store
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5psp226traits5PSP2216_approve_from_to17hc115ddc6234276ffE (type 10) (param i32 i32 i32 i64 i64)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 320
    i32.sub
    local.tee 5
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          call $_ZN140_$LT$$LT$ink_env..types..DefaultEnvironment$u20$as$u20$ink_env..types..Environment$GT$..AccountId$u20$as$u20$brush..traits..AccountIdExt$GT$7is_zero17hd36245406920208bE
          br_if 0 (;@3;)
          local.get 2
          call $_ZN140_$LT$$LT$ink_env..types..DefaultEnvironment$u20$as$u20$ink_env..types..Environment$GT$..AccountId$u20$as$u20$brush..traits..AccountIdExt$GT$7is_zero17hd36245406920208bE
          br_if 0 (;@3;)
          local.get 5
          i32.const 24
          i32.add
          local.get 1
          i32.const 24
          i32.add
          i64.load align=1
          i64.store
          local.get 5
          i32.const 16
          i32.add
          local.get 1
          i32.const 16
          i32.add
          i64.load align=1
          i64.store
          local.get 5
          i32.const 8
          i32.add
          local.get 1
          i32.const 8
          i32.add
          i64.load align=1
          i64.store
          local.get 5
          i32.const 40
          i32.add
          local.get 2
          i32.const 8
          i32.add
          i64.load align=1
          i64.store
          local.get 5
          i32.const 48
          i32.add
          local.get 2
          i32.const 16
          i32.add
          i64.load align=1
          i64.store
          local.get 5
          i32.const 56
          i32.add
          local.get 2
          i32.const 24
          i32.add
          i64.load align=1
          i64.store
          local.get 5
          local.get 1
          i64.load align=1
          i64.store
          local.get 5
          local.get 2
          i64.load align=1
          i64.store offset=32
          block  ;; label = @4
            local.get 0
            i32.const 352
            i32.add
            local.get 5
            call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$11lazily_load17ha761820488558d57E
            local.tee 1
            i64.load
            i64.const 1
            i64.ne
            br_if 0 (;@4;)
            local.get 1
            local.get 3
            i64.store offset=8
            local.get 1
            i32.const 0
            i32.store8 offset=32
            local.get 1
            i32.const 16
            i32.add
            local.get 4
            i64.store
            br 3 (;@1;)
          end
          local.get 5
          i32.const 64
          i32.add
          local.get 5
          i32.const 64
          call $memcpy
          drop
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 0
                                i32.const 284
                                i32.add
                                i32.load
                                local.tee 2
                                local.get 0
                                i32.const 288
                                i32.add
                                i32.load
                                i32.eq
                                br_if 0 (;@14;)
                                local.get 0
                                i32.const 296
                                i32.add
                                local.tee 6
                                local.get 0
                                i32.const 280
                                i32.add
                                i32.load
                                local.tee 2
                                call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$11lazily_load17h5239fbf66e51cb1fE
                                local.set 1
                                local.get 5
                                i32.const 1
                                i32.store8 offset=152
                                local.get 5
                                i32.const 152
                                i32.add
                                i32.const 1
                                i32.or
                                local.get 5
                                i32.const 64
                                i32.add
                                i32.const 64
                                call $memcpy
                                drop
                                local.get 5
                                i32.const 240
                                i32.add
                                local.get 1
                                local.get 5
                                i32.const 152
                                i32.add
                                call $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$3put17hc4decdb7e8d67aaeE
                                local.get 5
                                i32.load8_u offset=240
                                local.tee 1
                                i32.const 2
                                i32.eq
                                br_if 8 (;@6;)
                                local.get 1
                                i32.const 1
                                i32.eq
                                br_if 11 (;@3;)
                                local.get 5
                                i32.load offset=244
                                local.set 1
                                block  ;; label = @15
                                  local.get 5
                                  i32.const 248
                                  i32.add
                                  i32.load
                                  local.tee 7
                                  local.get 2
                                  i32.ne
                                  br_if 0 (;@15;)
                                  local.get 1
                                  local.get 2
                                  i32.eq
                                  br_if 4 (;@11;)
                                end
                                local.get 6
                                local.get 7
                                call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$7get_mut17h6f3f997cff70f2ecE
                                local.tee 8
                                br_if 1 (;@13;)
                                i32.const 0
                                local.set 8
                                br 2 (;@12;)
                              end
                              local.get 5
                              i32.const 128
                              i32.add
                              local.get 0
                              i32.const 336
                              i32.add
                              local.get 2
                              call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17hfb07304c0de55b1cE
                              block  ;; label = @14
                                block  ;; label = @15
                                  local.get 5
                                  i32.load offset=128
                                  i32.const 1
                                  i32.eq
                                  br_if 0 (;@15;)
                                  local.get 5
                                  i32.const 168
                                  i32.add
                                  local.get 5
                                  i32.const 148
                                  i32.add
                                  i32.load
                                  i32.store
                                  local.get 5
                                  i32.const 160
                                  i32.add
                                  local.get 5
                                  i32.const 140
                                  i32.add
                                  i64.load align=4
                                  i64.store
                                  local.get 5
                                  local.get 5
                                  i64.load offset=132 align=4
                                  i64.store offset=152
                                  i32.const 72
                                  i32.const 4
                                  call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E
                                  local.tee 1
                                  i32.const 1
                                  i32.store8
                                  local.get 1
                                  i32.const 1
                                  i32.add
                                  local.get 5
                                  i32.const 64
                                  i32.add
                                  i32.const 64
                                  call $memcpy
                                  drop
                                  local.get 1
                                  i32.const 0
                                  i32.store8 offset=68
                                  local.get 5
                                  i32.const 152
                                  i32.add
                                  local.get 1
                                  call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17h4e24b0c0e3e4d4f8E
                                  drop
                                  br 1 (;@14;)
                                end
                                local.get 5
                                i32.const 136
                                i32.add
                                i32.load
                                local.get 5
                                i32.const 140
                                i32.add
                                i32.load
                                i32.const 2
                                i32.shl
                                i32.add
                                i32.const 48
                                i32.add
                                i32.load
                                local.set 1
                                local.get 5
                                i32.const 1
                                i32.store8 offset=152
                                local.get 5
                                i32.const 152
                                i32.add
                                i32.const 1
                                i32.or
                                local.get 5
                                i32.const 64
                                i32.add
                                i32.const 64
                                call $memcpy
                                drop
                                local.get 5
                                i32.const 240
                                i32.add
                                local.get 1
                                local.get 5
                                i32.const 152
                                i32.add
                                call $_ZN11ink_storage4lazy5entry21StorageEntry$LT$T$GT$3put17hc4decdb7e8d67aaeE
                              end
                              local.get 0
                              i32.const 280
                              i32.add
                              i32.load
                              local.tee 1
                              i32.const 1
                              i32.add
                              local.tee 7
                              local.get 1
                              i32.lt_u
                              br_if 10 (;@3;)
                              local.get 0
                              local.get 7
                              i32.store offset=280
                              local.get 0
                              i32.load offset=288
                              local.tee 1
                              i32.const 1
                              i32.add
                              local.tee 7
                              local.get 1
                              i32.lt_u
                              br_if 10 (;@3;)
                              local.get 0
                              local.get 7
                              i32.store offset=288
                              br 9 (;@4;)
                            end
                            i32.const 0
                            local.get 8
                            i32.const 4
                            i32.add
                            local.get 8
                            i32.load8_u
                            i32.const 1
                            i32.eq
                            select
                            local.set 8
                          end
                          local.get 8
                          call $_ZN4core6option15Option$LT$T$GT$6expect17hbfd96ec228071fcaE
                          local.set 8
                          local.get 7
                          local.get 1
                          i32.eq
                          br_if 3 (;@8;)
                          local.get 8
                          local.get 1
                          i32.store
                          local.get 6
                          local.get 1
                          call $_ZN11ink_storage4lazy9lazy_imap21LazyIndexMap$LT$V$GT$7get_mut17h6f3f997cff70f2ecE
                          local.tee 6
                          br_if 1 (;@10;)
                          i32.const 0
                          local.set 6
                          br 2 (;@9;)
                        end
                        local.get 0
                        i32.load offset=284
                        local.set 1
                        br 5 (;@5;)
                      end
                      i32.const 0
                      local.get 6
                      i32.const 4
                      i32.add
                      local.get 6
                      i32.load8_u
                      i32.const 1
                      i32.eq
                      select
                      local.set 6
                    end
                    local.get 6
                    call $_ZN4core6option15Option$LT$T$GT$6expect17hbfd96ec228071fcaE
                    i32.const 4
                    i32.add
                    local.set 8
                    br 1 (;@7;)
                  end
                  local.get 8
                  local.get 1
                  i32.store offset=4
                end
                local.get 8
                local.get 7
                i32.store
                local.get 0
                i32.load offset=280
                local.get 2
                i32.ne
                br_if 2 (;@4;)
                local.get 1
                local.get 7
                local.get 7
                local.get 1
                i32.gt_u
                select
                local.set 1
                br 1 (;@5;)
              end
              call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
              unreachable
            end
            local.get 0
            local.get 1
            i32.store offset=280
          end
          local.get 0
          i32.load offset=284
          local.tee 1
          i32.const 1
          i32.add
          local.tee 7
          local.get 1
          i32.ge_u
          br_if 1 (;@2;)
        end
        unreachable
        unreachable
      end
      local.get 0
      local.get 7
      i32.store offset=284
      local.get 5
      i32.const 64
      i32.add
      local.get 5
      i32.const 64
      call $memcpy
      drop
      i32.const 40
      i32.const 8
      call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E
      local.tee 1
      local.get 3
      i64.store offset=8
      local.get 1
      i32.const 0
      i32.store8 offset=32
      local.get 1
      local.get 2
      i32.store offset=24
      local.get 1
      i64.const 1
      i64.store
      local.get 1
      i32.const 16
      i32.add
      local.get 4
      i64.store
      local.get 5
      i32.const 240
      i32.add
      local.get 5
      i32.const 64
      i32.add
      i32.const 64
      call $memcpy
      drop
      local.get 5
      i32.const 152
      i32.add
      local.get 0
      i32.const 392
      i32.add
      local.get 5
      i32.const 240
      i32.add
      call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17h367e8cc61234b4d4E
      block  ;; label = @2
        local.get 5
        i32.load offset=152
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        local.get 5
        i32.const 240
        i32.add
        local.get 5
        i32.const 152
        i32.add
        i32.const 4
        i32.or
        i32.const 80
        call $memcpy
        drop
        local.get 5
        i32.const 240
        i32.add
        local.get 1
        call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17h05eaa6f42b80d554E
        drop
        br 1 (;@1;)
      end
      local.get 5
      i32.const 152
      i32.add
      i32.const 8
      i32.add
      i32.load
      local.get 5
      i32.const 164
      i32.add
      i32.load
      i32.const 2
      i32.shl
      i32.add
      i32.const 4
      i32.add
      local.get 1
      i32.store
    end
    local.get 5
    i32.const 320
    i32.add
    global.set 0)
  (func $_ZN140_$LT$$LT$ink_env..types..DefaultEnvironment$u20$as$u20$ink_env..types..Environment$GT$..AccountId$u20$as$u20$brush..traits..AccountIdExt$GT$7is_zero17hd36245406920208bE (type 7) (param i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 24
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i32.const 16
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i32.const 8
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i64.const 0
    i64.store
    local.get 0
    local.get 1
    call $_ZN69_$LT$T$u20$as$u20$core..array..equality..SpecArrayEq$LT$U$C$_$GT$$GT$7spec_eq17heb72ec737bd773c4E
    local.set 0
    local.get 1
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$11lazily_load17ha761820488558d57E (type 1) (param i32 i32) (result i32)
    (local i32 i64 i32 i32 i32 i64)
    global.get 0
    i32.const 240
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 128
    i32.add
    local.get 1
    i32.const 64
    call $memcpy
    drop
    local.get 2
    i32.const 8
    i32.add
    local.get 0
    i32.const 40
    i32.add
    local.get 2
    i32.const 128
    i32.add
    call $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17h367e8cc61234b4d4E
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load offset=8
        i32.const 1
        i32.eq
        br_if 0 (;@2;)
        i64.const 2
        local.set 3
        block  ;; label = @3
          local.get 0
          i64.load
          i64.const 1
          i64.ne
          br_if 0 (;@3;)
          local.get 2
          i32.const 208
          i32.add
          i32.const 24
          i32.add
          local.tee 4
          local.get 0
          i32.const 32
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 208
          i32.add
          i32.const 16
          i32.add
          local.tee 5
          local.get 0
          i32.const 24
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 208
          i32.add
          i32.const 8
          i32.add
          local.tee 6
          local.get 0
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 0
          i64.load offset=8
          i64.store offset=208
          local.get 2
          i32.const 128
          i32.add
          i32.const 8
          i32.add
          local.get 2
          i32.const 208
          i32.add
          local.get 1
          call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$13to_offset_key17h7fe2167c13a89784E
          local.get 4
          local.get 2
          i32.const 128
          i32.add
          i32.const 32
          i32.add
          i64.load
          i64.store
          local.get 5
          local.get 2
          i32.const 128
          i32.add
          i32.const 24
          i32.add
          i64.load
          i64.store
          local.get 6
          local.get 2
          i32.const 128
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 2
          i64.load offset=136
          i64.store offset=208
          local.get 2
          i32.const 96
          i32.add
          local.get 2
          i32.const 208
          i32.add
          call $_ZN11ink_storage6traits7optspec20pull_packed_root_opt17ha3f29b1c536f4bc0E
          local.get 2
          i64.load offset=96
          local.set 3
        end
        local.get 2
        i32.const 8
        i32.add
        i32.const 4
        i32.or
        local.set 0
        i64.const 0
        local.set 7
        block  ;; label = @3
          local.get 3
          i64.const 2
          i64.eq
          br_if 0 (;@3;)
          local.get 2
          i32.const 208
          i32.add
          i32.const 16
          i32.add
          local.get 2
          i32.const 120
          i32.add
          i64.load
          i64.store
          local.get 2
          i32.const 216
          i32.add
          local.get 2
          i32.const 96
          i32.add
          i32.const 16
          i32.add
          i64.load
          i64.store
          local.get 2
          local.get 2
          i64.load offset=104
          i64.store offset=208
          local.get 3
          local.set 7
        end
        local.get 2
        i32.const 128
        i32.add
        local.get 0
        i32.const 80
        call $memcpy
        drop
        i32.const 40
        i32.const 8
        call $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E.240
        local.tee 0
        local.get 7
        i64.store
        local.get 0
        i32.const 1
        i32.store8 offset=32
        local.get 0
        local.get 2
        i64.load offset=208
        i64.store offset=8
        local.get 0
        i32.const 16
        i32.add
        local.get 2
        i32.const 208
        i32.add
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 0
        i32.const 24
        i32.add
        local.get 2
        i32.const 208
        i32.add
        i32.const 16
        i32.add
        i64.load
        i64.store
        local.get 2
        i32.const 128
        i32.add
        local.get 0
        call $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17h05eaa6f42b80d554E
        i32.load
        local.set 0
        br 1 (;@1;)
      end
      local.get 2
      i32.const 16
      i32.add
      i32.load
      local.get 2
      i32.const 20
      i32.add
      i32.load
      i32.const 2
      i32.shl
      i32.add
      i32.const 4
      i32.add
      i32.load
      local.set 0
    end
    local.get 2
    i32.const 240
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17hfb07304c0de55b1cE (type 2) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17h084a0905094048e2E
      local.tee 3
      i32.store offset=4
      i32.const 0
      local.set 4
      local.get 1
      i32.const 0
      i32.store
    end
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.load16_u offset=94
        local.tee 5
        i32.const 2
        i32.shl
        local.set 6
        i32.const 0
        local.set 7
        i32.const -1
        local.set 8
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 6
              local.get 7
              i32.ne
              br_if 0 (;@5;)
              local.get 5
              local.set 8
              br 2 (;@3;)
            end
            local.get 3
            local.get 7
            i32.add
            local.set 9
            local.get 8
            i32.const 1
            i32.add
            local.set 8
            local.get 7
            i32.const 4
            i32.add
            local.set 7
            block  ;; label = @5
              i32.const -1
              local.get 9
              i32.const 4
              i32.add
              i32.load
              local.tee 9
              local.get 2
              i32.ne
              local.get 9
              local.get 2
              i32.gt_u
              select
              i32.const 1
              i32.add
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
          end
          local.get 0
          i32.const 12
          i32.add
          local.get 8
          i32.store
          local.get 0
          i32.const 8
          i32.add
          local.get 3
          i32.store
          local.get 0
          i32.const 16
          i32.add
          local.set 7
          i32.const 1
          local.set 8
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const -1
          i32.add
          local.set 4
          local.get 3
          local.get 8
          i32.const 2
          i32.shl
          i32.add
          i32.const 96
          i32.add
          i32.load
          local.set 3
          br 1 (;@2;)
        end
      end
      local.get 0
      i32.const 16
      i32.add
      local.get 8
      i32.store
      local.get 0
      i32.const 12
      i32.add
      local.get 3
      i32.store
      i32.const 0
      local.set 8
      local.get 0
      i32.const 8
      i32.add
      i32.const 0
      i32.store
      local.get 0
      i32.const 20
      i32.add
      local.set 7
      local.get 2
      local.set 4
    end
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 7
    local.get 1
    i32.store
    local.get 0
    local.get 8
    i32.store)
  (func $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17h4e24b0c0e3e4d4f8E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 3
    local.get 2
    i32.const 48
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 12
    i32.add
    i32.load
    i32.store
    local.get 2
    local.get 0
    i64.load offset=4 align=4
    i64.store offset=48
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.load offset=52
                  local.tee 4
                  i32.load16_u offset=94
                  i32.const 11
                  i32.lt_u
                  br_if 0 (;@7;)
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 2
                  i32.load offset=56
                  call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
                  local.get 2
                  i32.const 64
                  i32.add
                  i32.const 8
                  i32.add
                  i32.load
                  local.set 5
                  local.get 2
                  i32.load offset=68
                  local.set 6
                  local.get 2
                  i32.load offset=64
                  local.set 7
                  local.get 2
                  i32.load offset=48
                  local.set 8
                  call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17h084a0905094048e2E
                  local.set 9
                  local.get 4
                  i32.load16_u offset=94
                  local.tee 10
                  local.get 7
                  i32.sub
                  local.tee 11
                  local.get 10
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 11
                  i32.const -1
                  i32.add
                  local.tee 12
                  local.get 11
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 9
                  local.get 12
                  i32.store16 offset=94
                  local.get 7
                  i32.const 1
                  i32.add
                  local.tee 13
                  local.get 7
                  i32.lt_u
                  br_if 6 (;@1;)
                  local.get 10
                  local.get 13
                  i32.sub
                  local.tee 11
                  local.get 10
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 4
                  local.get 7
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 10
                  i32.const 4
                  i32.add
                  i32.load
                  local.set 14
                  local.get 10
                  i32.const 48
                  i32.add
                  i32.load
                  local.set 15
                  local.get 2
                  i32.const 40
                  i32.add
                  local.get 9
                  i32.const 4
                  i32.add
                  local.get 12
                  call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h60030e9adf15a33cE
                  local.get 11
                  local.get 2
                  i32.load offset=44
                  i32.ne
                  br_if 6 (;@1;)
                  local.get 2
                  i32.load offset=40
                  local.get 4
                  local.get 13
                  i32.const 2
                  i32.shl
                  i32.add
                  local.tee 10
                  i32.const 4
                  i32.add
                  local.get 11
                  i32.const 2
                  i32.shl
                  local.tee 13
                  call $memcpy
                  drop
                  local.get 2
                  i32.const 32
                  i32.add
                  local.get 9
                  i32.const 48
                  i32.add
                  local.get 12
                  call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17he4fcea40c4f0de45E
                  local.get 11
                  local.get 2
                  i32.load offset=36
                  i32.ne
                  br_if 6 (;@1;)
                  local.get 2
                  i32.load offset=32
                  local.get 10
                  i32.const 48
                  i32.add
                  local.get 13
                  call $memcpy
                  drop
                  local.get 4
                  local.get 7
                  i32.store16 offset=94
                  local.get 2
                  local.get 5
                  i32.store offset=72
                  local.get 2
                  local.get 9
                  local.get 4
                  local.get 6
                  select
                  i32.store offset=68
                  i32.const 0
                  local.set 10
                  local.get 2
                  i32.const 0
                  local.get 8
                  local.get 6
                  select
                  i32.store offset=64
                  local.get 2
                  i32.const 64
                  i32.add
                  local.get 3
                  local.get 1
                  call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hbfcf1d7d9f7ad73eE
                  local.set 16
                  loop  ;; label = @8
                    local.get 4
                    i32.load
                    local.tee 7
                    i32.eqz
                    br_if 2 (;@6;)
                    local.get 8
                    i32.const 1
                    i32.add
                    local.tee 1
                    local.get 8
                    i32.lt_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 4
                    i32.load16_u offset=92
                    local.tee 8
                    i32.store offset=56
                    local.get 2
                    local.get 7
                    i32.store offset=52
                    local.get 2
                    local.get 1
                    i32.store offset=48
                    local.get 1
                    i32.const -1
                    i32.add
                    local.tee 4
                    local.get 1
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 4
                    local.get 10
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 7
                    i32.load16_u offset=94
                    i32.const 11
                    i32.lt_u
                    br_if 4 (;@4;)
                    local.get 2
                    i32.const 64
                    i32.add
                    local.get 8
                    call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
                    local.get 2
                    i32.load offset=72
                    local.set 13
                    local.get 2
                    i32.load offset=68
                    local.set 5
                    local.get 2
                    i32.load offset=64
                    local.set 8
                    local.get 7
                    i32.load16_u offset=94
                    local.set 6
                    call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h801bf85f2302c549E
                    local.set 4
                    local.get 7
                    i32.load16_u offset=94
                    local.tee 10
                    local.get 8
                    i32.sub
                    local.tee 12
                    local.get 10
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 12
                    i32.const -1
                    i32.add
                    local.tee 3
                    local.get 12
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 4
                    local.get 3
                    i32.store16 offset=94
                    local.get 8
                    i32.const 1
                    i32.add
                    local.tee 12
                    local.get 8
                    i32.lt_u
                    br_if 7 (;@1;)
                    local.get 10
                    local.get 12
                    i32.sub
                    local.tee 11
                    local.get 10
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 7
                    local.get 8
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 10
                    i32.const 4
                    i32.add
                    i32.load
                    local.set 17
                    local.get 10
                    i32.const 48
                    i32.add
                    i32.load
                    local.set 18
                    local.get 2
                    i32.const 24
                    i32.add
                    local.get 4
                    i32.const 4
                    i32.add
                    local.get 3
                    call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h60030e9adf15a33cE
                    local.get 11
                    local.get 2
                    i32.load offset=28
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 2
                    i32.load offset=24
                    local.get 7
                    local.get 12
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 10
                    i32.const 4
                    i32.add
                    local.get 11
                    i32.const 2
                    i32.shl
                    local.tee 19
                    call $memcpy
                    drop
                    local.get 2
                    i32.const 16
                    i32.add
                    local.get 4
                    i32.const 48
                    i32.add
                    local.get 3
                    call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17he4fcea40c4f0de45E
                    local.get 11
                    local.get 2
                    i32.load offset=20
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 2
                    i32.load offset=16
                    local.get 10
                    i32.const 48
                    i32.add
                    local.get 19
                    call $memcpy
                    drop
                    local.get 7
                    local.get 8
                    i32.store16 offset=94
                    local.get 6
                    i32.const 1
                    i32.add
                    local.tee 3
                    local.get 12
                    i32.sub
                    local.tee 8
                    local.get 3
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 4
                    i32.load16_u offset=94
                    local.tee 3
                    i32.const 12
                    i32.ge_u
                    br_if 3 (;@5;)
                    local.get 8
                    local.get 3
                    i32.const 1
                    i32.add
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 4
                    i32.const 96
                    i32.add
                    local.get 10
                    i32.const 96
                    i32.add
                    local.get 8
                    i32.const 2
                    i32.shl
                    call $memcpy
                    drop
                    local.get 2
                    i32.const 8
                    i32.add
                    local.get 4
                    local.get 1
                    call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h360c1fbf92f55ce4E
                    local.get 2
                    i32.load offset=12
                    local.set 4
                    local.get 2
                    i32.load offset=8
                    local.set 10
                    local.get 1
                    local.set 8
                    local.get 7
                    local.set 3
                    block  ;; label = @9
                      local.get 5
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 10
                      local.set 8
                      local.get 4
                      local.set 3
                    end
                    local.get 2
                    local.get 13
                    i32.store offset=72
                    local.get 2
                    local.get 3
                    i32.store offset=68
                    local.get 2
                    local.get 8
                    i32.store offset=64
                    local.get 2
                    i32.const 64
                    i32.add
                    local.get 14
                    local.get 15
                    local.get 9
                    call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hd94233843f500156E
                    local.get 1
                    local.set 8
                    local.get 4
                    local.set 9
                    local.get 18
                    local.set 15
                    local.get 17
                    local.set 14
                    local.get 7
                    local.set 4
                    br 0 (;@8;)
                  end
                end
                local.get 2
                i32.const 48
                i32.add
                local.get 3
                local.get 1
                call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hbfcf1d7d9f7ad73eE
                local.set 16
                br 3 (;@3;)
              end
              local.get 0
              i32.load offset=16
              local.tee 1
              i32.load offset=4
              local.tee 4
              i32.eqz
              br_if 4 (;@1;)
              local.get 1
              i32.load
              local.set 7
              call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h801bf85f2302c549E
              local.tee 8
              local.get 4
              i32.store offset=96
              local.get 7
              i32.const 1
              i32.add
              local.tee 4
              local.get 7
              i32.lt_u
              br_if 4 (;@1;)
              local.get 2
              local.get 8
              local.get 4
              call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h360c1fbf92f55ce4E
              local.get 2
              i32.load
              local.set 4
              local.get 1
              local.get 2
              i32.load offset=4
              local.tee 7
              i32.store offset=4
              local.get 1
              local.get 4
              i32.store
              local.get 4
              i32.const -1
              i32.add
              local.tee 8
              local.get 4
              i32.gt_u
              br_if 4 (;@1;)
              local.get 8
              local.get 10
              i32.ne
              br_if 4 (;@1;)
              local.get 7
              i32.load16_u offset=94
              local.tee 8
              i32.const 10
              i32.gt_u
              br_if 4 (;@1;)
              local.get 7
              local.get 8
              i32.const 1
              i32.add
              local.tee 10
              i32.store16 offset=94
              local.get 7
              local.get 8
              i32.const 2
              i32.shl
              i32.add
              local.tee 8
              i32.const 48
              i32.add
              local.get 15
              i32.store
              local.get 8
              i32.const 4
              i32.add
              local.get 14
              i32.store
              local.get 7
              local.get 10
              i32.const 2
              i32.shl
              i32.add
              i32.const 96
              i32.add
              local.get 9
              i32.store
              local.get 2
              local.get 10
              i32.store offset=72
              local.get 2
              local.get 7
              i32.store offset=68
              local.get 2
              local.get 4
              i32.store offset=64
              local.get 2
              i32.const 64
              i32.add
              call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6d015cf587f65ed9E
              local.get 1
              i32.load offset=8
              local.tee 4
              i32.const 1
              i32.add
              local.tee 7
              local.get 4
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              i32.const 8
              i32.add
              local.set 1
              br 3 (;@2;)
            end
            call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
            unreachable
          end
          local.get 2
          i32.const 48
          i32.add
          local.get 14
          local.get 15
          local.get 9
          call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hd94233843f500156E
        end
        local.get 0
        i32.load offset=16
        local.tee 4
        i32.load offset=8
        local.tee 1
        i32.const 1
        i32.add
        local.tee 7
        local.get 1
        i32.lt_u
        br_if 1 (;@1;)
        local.get 4
        i32.const 8
        i32.add
        local.set 1
      end
      local.get 1
      local.get 7
      i32.store
      local.get 2
      i32.const 80
      i32.add
      global.set 0
      local.get 16
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree3map21BTreeMap$LT$K$C$V$GT$5entry17h367e8cc61234b4d4E (type 2) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load offset=4
        local.tee 3
        i32.eqz
        br_if 0 (;@2;)
        local.get 1
        i32.load
        local.set 4
        br 1 (;@1;)
      end
      local.get 1
      call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17h669c0a87dfa579baE
      local.tee 3
      i32.store offset=4
      i32.const 0
      local.set 4
      local.get 1
      i32.const 0
      i32.store
    end
    local.get 2
    i32.const 32
    i32.add
    local.set 5
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.load16_u offset=50
        local.tee 6
        i32.const 6
        i32.shl
        local.set 7
        i32.const 0
        local.set 8
        i32.const -1
        local.set 9
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 7
              local.get 8
              i32.ne
              br_if 0 (;@5;)
              local.get 6
              local.set 9
              br 2 (;@3;)
            end
            block  ;; label = @5
              local.get 2
              local.get 3
              local.get 8
              i32.add
              local.tee 10
              i32.const 52
              i32.add
              call $_ZN60_$LT$ink_env..types..AccountId$u20$as$u20$core..cmp..Ord$GT$3cmp17h3e35a495ccaba03eE
              local.tee 11
              i32.const 255
              i32.and
              br_if 0 (;@5;)
              local.get 5
              local.get 10
              i32.const 84
              i32.add
              call $_ZN60_$LT$ink_env..types..AccountId$u20$as$u20$core..cmp..Ord$GT$3cmp17h3e35a495ccaba03eE
              local.set 11
            end
            local.get 9
            i32.const 1
            i32.add
            local.set 9
            local.get 8
            i32.const 64
            i32.add
            local.set 8
            block  ;; label = @5
              local.get 11
              i32.const 24
              i32.shl
              i32.const 24
              i32.shr_s
              i32.const 1
              i32.add
              br_table 2 (;@3;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
          end
          i32.const 1
          local.set 8
          br 2 (;@1;)
        end
        block  ;; label = @3
          local.get 4
          i32.eqz
          br_if 0 (;@3;)
          local.get 4
          i32.const -1
          i32.add
          local.set 4
          local.get 3
          local.get 9
          i32.const 2
          i32.shl
          i32.add
          i32.const 756
          i32.add
          i32.load
          local.set 3
          br 1 (;@2;)
        end
      end
      local.get 0
      i32.const 20
      i32.add
      local.get 2
      i32.const 64
      call $memcpy
      drop
      i32.const 0
      local.set 4
      i32.const 0
      local.set 8
    end
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 0
    local.get 8
    i32.store
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i32.store
    local.get 0
    i32.const 12
    i32.add
    local.get 9
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 3
    i32.store)
  (func $_ZN5alloc11collections5btree3map5entry24VacantEntry$LT$K$C$V$GT$6insert17h05eaa6f42b80d554E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 448
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 56
    i32.add
    i32.const 8
    i32.add
    local.get 0
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 2
    local.get 0
    i64.load align=4
    i64.store offset=56
    local.get 2
    i32.const 120
    i32.add
    local.get 0
    i32.const 16
    i32.add
    local.tee 3
    i32.const 64
    call $memcpy
    drop
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load offset=60
          local.tee 4
          i32.load16_u offset=50
          local.tee 5
          i32.const 11
          i32.lt_u
          br_if 0 (;@3;)
          local.get 2
          i32.const 384
          i32.add
          local.get 2
          i32.load offset=64
          call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
          local.get 2
          i32.const 384
          i32.add
          i32.const 8
          i32.add
          i32.load
          local.set 6
          local.get 2
          i32.load offset=388
          local.set 7
          local.get 2
          i32.load offset=384
          local.set 3
          local.get 2
          i32.load offset=56
          local.set 8
          call $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17h669c0a87dfa579baE
          local.set 9
          local.get 4
          i32.load16_u offset=50
          local.tee 10
          local.get 3
          i32.sub
          local.tee 11
          local.get 10
          i32.gt_u
          br_if 2 (;@1;)
          local.get 11
          i32.const -1
          i32.add
          local.tee 12
          local.get 11
          i32.gt_u
          br_if 2 (;@1;)
          local.get 9
          local.get 12
          i32.store16 offset=50
          local.get 4
          local.get 3
          i32.const 6
          i32.shl
          i32.add
          local.tee 11
          i32.const 52
          i32.add
          i32.load align=1
          local.set 13
          local.get 2
          i32.const 384
          i32.add
          local.get 11
          i32.const 56
          i32.add
          i32.const 60
          call $memcpy
          drop
          local.get 3
          i32.const 1
          i32.add
          local.tee 14
          local.get 3
          i32.lt_u
          br_if 2 (;@1;)
          local.get 10
          local.get 14
          i32.sub
          local.tee 11
          local.get 10
          i32.gt_u
          br_if 2 (;@1;)
          local.get 4
          local.get 3
          i32.const 2
          i32.shl
          i32.add
          i32.const 4
          i32.add
          i32.load
          local.set 15
          local.get 2
          i32.const 48
          i32.add
          local.get 9
          i32.const 52
          i32.add
          local.get 12
          call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h26193625bee7f760E
          local.get 11
          local.get 2
          i32.load offset=52
          i32.ne
          br_if 2 (;@1;)
          local.get 2
          i32.load offset=48
          local.get 4
          local.get 14
          i32.const 6
          i32.shl
          i32.add
          i32.const 52
          i32.add
          local.get 11
          i32.const 6
          i32.shl
          call $memcpy
          drop
          local.get 2
          i32.const 40
          i32.add
          local.get 9
          i32.const 4
          i32.add
          local.get 12
          call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17hda973abfff99f4dfE
          local.get 11
          local.get 2
          i32.load offset=44
          i32.ne
          br_if 2 (;@1;)
          local.get 2
          i32.load offset=40
          local.get 4
          local.get 14
          i32.const 2
          i32.shl
          i32.add
          i32.const 4
          i32.add
          local.get 11
          i32.const 2
          i32.shl
          call $memcpy
          drop
          local.get 4
          local.get 3
          i32.store16 offset=50
          local.get 2
          i32.const 320
          i32.add
          local.get 2
          i32.const 384
          i32.add
          i32.const 60
          call $memcpy
          drop
          local.get 2
          local.get 6
          i32.store offset=192
          local.get 2
          local.get 9
          local.get 4
          local.get 7
          select
          i32.store offset=188
          local.get 2
          i32.const 0
          local.get 8
          local.get 7
          select
          i32.store offset=184
          local.get 2
          i32.const 384
          i32.add
          local.get 2
          i32.const 120
          i32.add
          i32.const 64
          call $memcpy
          drop
          local.get 2
          i32.const 184
          i32.add
          local.get 2
          i32.const 384
          i32.add
          local.get 1
          call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17he03aa9c1ee608749E
          local.set 16
          local.get 2
          i32.const 260
          i32.add
          local.get 2
          i32.const 320
          i32.add
          i32.const 60
          call $memcpy
          drop
          br 1 (;@2;)
        end
        local.get 2
        i32.const 384
        i32.add
        local.get 3
        i32.const 64
        call $memcpy
        drop
        local.get 2
        i32.const 56
        i32.add
        local.get 2
        i32.const 384
        i32.add
        local.get 1
        call $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17he03aa9c1ee608749E
        local.set 16
        local.get 2
        i32.load offset=64
        local.set 13
        local.get 2
        i32.load offset=56
        local.set 8
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 5
          i32.const 10
          i32.le_u
          br_if 0 (;@3;)
          local.get 2
          i32.const 56
          i32.add
          local.get 2
          i32.const 260
          i32.add
          i32.const 60
          call $memcpy
          drop
          local.get 2
          i32.const 384
          i32.add
          i32.const 4
          i32.or
          local.set 6
          i32.const 0
          local.set 1
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 4
                    i32.load
                    local.tee 3
                    i32.eqz
                    br_if 0 (;@8;)
                    local.get 8
                    i32.const 1
                    i32.add
                    local.tee 11
                    local.get 8
                    i32.lt_u
                    br_if 7 (;@1;)
                    local.get 2
                    local.get 4
                    i32.load16_u offset=48
                    local.tee 5
                    i32.store offset=256
                    local.get 2
                    local.get 3
                    i32.store offset=252
                    local.get 2
                    local.get 11
                    i32.store offset=248
                    local.get 2
                    i32.const 260
                    i32.add
                    local.get 2
                    i32.const 56
                    i32.add
                    i32.const 60
                    call $memcpy
                    drop
                    local.get 11
                    i32.const -1
                    i32.add
                    local.tee 4
                    local.get 11
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 4
                    local.get 1
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 3
                    i32.load16_u offset=50
                    i32.const 11
                    i32.lt_u
                    local.tee 12
                    br_if 2 (;@6;)
                    local.get 2
                    i32.const 384
                    i32.add
                    local.get 5
                    call $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E
                    local.get 2
                    i32.load offset=392
                    local.set 17
                    local.get 2
                    i32.load offset=388
                    local.set 18
                    local.get 2
                    i32.load offset=384
                    local.set 4
                    local.get 3
                    i32.load16_u offset=50
                    local.set 19
                    call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h19f31af566aba95bE
                    local.set 8
                    local.get 3
                    i32.load16_u offset=50
                    local.tee 14
                    local.get 4
                    i32.sub
                    local.tee 5
                    local.get 14
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 5
                    i32.const -1
                    i32.add
                    local.tee 7
                    local.get 5
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 8
                    local.get 7
                    i32.store16 offset=50
                    local.get 3
                    local.get 4
                    i32.const 6
                    i32.shl
                    i32.add
                    local.tee 1
                    i32.const 52
                    i32.add
                    i32.load align=1
                    local.set 5
                    local.get 2
                    i32.const 384
                    i32.add
                    local.get 1
                    i32.const 56
                    i32.add
                    i32.const 60
                    call $memcpy
                    drop
                    local.get 4
                    i32.const 1
                    i32.add
                    local.tee 1
                    local.get 4
                    i32.lt_u
                    br_if 7 (;@1;)
                    local.get 14
                    local.get 1
                    i32.sub
                    local.tee 10
                    local.get 14
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 3
                    local.get 4
                    i32.const 2
                    i32.shl
                    i32.add
                    i32.const 4
                    i32.add
                    i32.load
                    local.set 14
                    local.get 2
                    i32.const 32
                    i32.add
                    local.get 8
                    i32.const 52
                    i32.add
                    local.get 7
                    call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h26193625bee7f760E
                    local.get 10
                    local.get 2
                    i32.load offset=36
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 2
                    i32.load offset=32
                    local.get 3
                    local.get 1
                    i32.const 6
                    i32.shl
                    i32.add
                    i32.const 52
                    i32.add
                    local.get 10
                    i32.const 6
                    i32.shl
                    call $memcpy
                    drop
                    local.get 2
                    i32.const 24
                    i32.add
                    local.get 8
                    i32.const 4
                    i32.add
                    local.get 7
                    call $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17hda973abfff99f4dfE
                    local.get 10
                    local.get 2
                    i32.load offset=28
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 2
                    i32.load offset=24
                    local.get 3
                    local.get 1
                    i32.const 2
                    i32.shl
                    i32.add
                    local.tee 7
                    i32.const 4
                    i32.add
                    local.get 10
                    i32.const 2
                    i32.shl
                    call $memcpy
                    drop
                    local.get 3
                    local.get 4
                    i32.store16 offset=50
                    local.get 2
                    i32.const 120
                    i32.add
                    local.get 2
                    i32.const 384
                    i32.add
                    i32.const 60
                    call $memcpy
                    drop
                    local.get 19
                    i32.const 1
                    i32.add
                    local.tee 10
                    local.get 1
                    i32.sub
                    local.tee 4
                    local.get 10
                    i32.gt_u
                    br_if 7 (;@1;)
                    local.get 8
                    i32.load16_u offset=50
                    local.tee 1
                    i32.const 12
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 4
                    local.get 1
                    i32.const 1
                    i32.add
                    i32.ne
                    br_if 7 (;@1;)
                    local.get 8
                    i32.const 756
                    i32.add
                    local.get 7
                    i32.const 756
                    i32.add
                    local.get 4
                    i32.const 2
                    i32.shl
                    call $memcpy
                    drop
                    local.get 2
                    i32.const 16
                    i32.add
                    local.get 8
                    local.get 11
                    call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h4750e4b371ff1a83E
                    local.get 2
                    i32.load offset=20
                    local.set 10
                    local.get 2
                    i32.load offset=16
                    local.set 1
                    local.get 2
                    i32.const 320
                    i32.add
                    local.get 2
                    i32.const 120
                    i32.add
                    i32.const 60
                    call $memcpy
                    drop
                    local.get 11
                    local.set 4
                    local.get 3
                    local.set 8
                    block  ;; label = @9
                      local.get 18
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 1
                      local.set 4
                      local.get 10
                      local.set 8
                    end
                    local.get 2
                    local.get 17
                    i32.store offset=128
                    local.get 2
                    local.get 8
                    i32.store offset=124
                    local.get 2
                    local.get 4
                    i32.store offset=120
                    local.get 2
                    local.get 13
                    i32.store offset=384
                    local.get 6
                    local.get 2
                    i32.const 260
                    i32.add
                    i32.const 60
                    call $memcpy
                    drop
                    local.get 2
                    i32.const 120
                    i32.add
                    local.get 2
                    i32.const 384
                    i32.add
                    local.get 15
                    local.get 9
                    call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17h3f7d3232dc6e2d8aE
                    local.get 2
                    i32.const 184
                    i32.add
                    local.get 2
                    i32.const 320
                    i32.add
                    i32.const 60
                    call $memcpy
                    drop
                    br 3 (;@5;)
                  end
                  local.get 2
                  i32.const 120
                  i32.add
                  local.get 2
                  i32.const 56
                  i32.add
                  i32.const 60
                  call $memcpy
                  drop
                  local.get 0
                  i32.load offset=12
                  local.tee 11
                  i32.load offset=4
                  local.tee 4
                  i32.eqz
                  br_if 6 (;@1;)
                  local.get 11
                  i32.load
                  local.set 3
                  call $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h19f31af566aba95bE
                  local.tee 8
                  local.get 4
                  i32.store offset=756
                  local.get 3
                  i32.const 1
                  i32.add
                  local.tee 4
                  local.get 3
                  i32.lt_u
                  br_if 6 (;@1;)
                  local.get 2
                  i32.const 8
                  i32.add
                  local.get 8
                  local.get 4
                  call $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h4750e4b371ff1a83E
                  local.get 2
                  i32.load offset=8
                  local.set 4
                  local.get 11
                  local.get 2
                  i32.load offset=12
                  local.tee 3
                  i32.store offset=4
                  local.get 11
                  local.get 4
                  i32.store
                  local.get 2
                  i32.const 384
                  i32.add
                  local.get 2
                  i32.const 120
                  i32.add
                  i32.const 60
                  call $memcpy
                  drop
                  local.get 4
                  i32.const -1
                  i32.add
                  local.tee 8
                  local.get 4
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 8
                  local.get 1
                  i32.ne
                  br_if 6 (;@1;)
                  local.get 3
                  i32.load16_u offset=50
                  local.tee 8
                  i32.const 10
                  i32.gt_u
                  br_if 6 (;@1;)
                  local.get 3
                  local.get 8
                  i32.const 1
                  i32.add
                  local.tee 5
                  i32.store16 offset=50
                  local.get 3
                  local.get 8
                  i32.const 6
                  i32.shl
                  i32.add
                  local.tee 1
                  i32.const 52
                  i32.add
                  local.get 13
                  i32.store align=1
                  local.get 1
                  i32.const 56
                  i32.add
                  local.get 2
                  i32.const 384
                  i32.add
                  i32.const 60
                  call $memcpy
                  drop
                  local.get 3
                  local.get 5
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 756
                  i32.add
                  local.get 9
                  i32.store
                  local.get 3
                  local.get 8
                  i32.const 2
                  i32.shl
                  i32.add
                  i32.const 4
                  i32.add
                  local.get 15
                  i32.store
                  local.get 2
                  local.get 5
                  i32.store offset=328
                  local.get 2
                  local.get 3
                  i32.store offset=324
                  local.get 2
                  local.get 4
                  i32.store offset=320
                  local.get 2
                  i32.const 320
                  i32.add
                  call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6179d7c09eaf4df4E
                  local.get 11
                  i32.load offset=8
                  local.tee 4
                  i32.const 1
                  i32.add
                  local.tee 3
                  local.get 4
                  i32.lt_u
                  br_if 6 (;@1;)
                  local.get 11
                  i32.const 8
                  i32.add
                  local.set 11
                  br 5 (;@2;)
                end
                call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
                unreachable
              end
              local.get 2
              local.get 13
              i32.store offset=384
              local.get 6
              local.get 2
              i32.const 56
              i32.add
              i32.const 60
              call $memcpy
              drop
              local.get 2
              i32.const 248
              i32.add
              local.get 2
              i32.const 384
              i32.add
              local.get 15
              local.get 9
              call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17h3f7d3232dc6e2d8aE
            end
            local.get 12
            br_if 1 (;@3;)
            local.get 2
            i32.const 56
            i32.add
            local.get 2
            i32.const 184
            i32.add
            i32.const 60
            call $memcpy
            drop
            local.get 11
            local.set 8
            local.get 5
            local.set 13
            local.get 10
            local.set 9
            local.get 14
            local.set 15
            local.get 3
            local.set 4
            br 0 (;@4;)
          end
        end
        local.get 0
        i32.load offset=12
        local.tee 4
        i32.load offset=8
        local.tee 11
        i32.const 1
        i32.add
        local.tee 3
        local.get 11
        i32.lt_u
        br_if 1 (;@1;)
        local.get 4
        i32.const 8
        i32.add
        local.set 11
      end
      local.get 11
      local.get 3
      i32.store
      local.get 2
      i32.const 448
      i32.add
      global.set 0
      local.get 16
      return
    end
    unreachable
    unreachable)
  (func $_ZN5psp226traits5PSP2217_transfer_from_to17h81f2dfe081d11fc0E (type 11) (param i32 i32 i32 i64 i64 i32)
    (local i32 i32 i32 i32 i64 i32 i64 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 352
    i32.sub
    local.tee 6
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        call $_ZN140_$LT$$LT$ink_env..types..DefaultEnvironment$u20$as$u20$ink_env..types..Environment$GT$..AccountId$u20$as$u20$brush..traits..AccountIdExt$GT$7is_zero17hd36245406920208bE
        br_if 0 (;@2;)
        local.get 2
        call $_ZN140_$LT$$LT$ink_env..types..DefaultEnvironment$u20$as$u20$ink_env..types..Environment$GT$..AccountId$u20$as$u20$brush..traits..AccountIdExt$GT$7is_zero17hd36245406920208bE
        br_if 0 (;@2;)
        local.get 6
        i32.const 160
        i32.add
        i32.const 24
        i32.add
        local.tee 7
        local.get 2
        i32.const 24
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 160
        i32.add
        i32.const 16
        i32.add
        local.tee 8
        local.get 2
        i32.const 16
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 160
        i32.add
        i32.const 8
        i32.add
        local.tee 9
        local.get 2
        i32.const 8
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        local.get 2
        i64.load align=1
        i64.store offset=160
        block  ;; label = @3
          local.get 6
          i32.const 160
          i32.add
          local.get 0
          i32.const 584
          i32.add
          call $_ZN69_$LT$T$u20$as$u20$core..array..equality..SpecArrayEq$LT$U$C$_$GT$$GT$7spec_eq17heb72ec737bd773c4E
          i32.eqz
          br_if 0 (;@3;)
          local.get 6
          i32.const 64
          i32.add
          i32.const 4
          i32.or
          call $_ZN76_$LT$alloc..string..String$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$4from17hefa3ad3efa3f0b3eE
          br 1 (;@2;)
        end
        local.get 7
        local.get 1
        i32.const 24
        i32.add
        i64.load align=1
        i64.store
        local.get 8
        local.get 1
        i32.const 16
        i32.add
        i64.load align=1
        i64.store
        local.get 9
        local.get 1
        i32.const 8
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        local.get 1
        i64.load align=1
        i64.store offset=160
        local.get 6
        i32.const 48
        i32.add
        local.get 0
        local.get 6
        i32.const 160
        i32.add
        call $_ZN5psp226traits5PSP2210balance_of17h8e8fb26b27704ba9E
        local.get 6
        i64.load offset=48
        local.tee 10
        local.get 3
        i64.lt_u
        local.tee 11
        local.get 6
        i32.const 48
        i32.add
        i32.const 8
        i32.add
        i64.load
        local.tee 12
        local.get 4
        i64.lt_u
        local.get 12
        local.get 4
        i64.eq
        select
        br_if 0 (;@2;)
        local.get 6
        i32.const 128
        i32.add
        call $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE
        local.get 6
        i32.const 216
        i32.add
        local.get 6
        i32.const 128
        i32.add
        i32.const 24
        i32.add
        i64.load
        i64.store
        local.get 6
        i32.const 208
        i32.add
        local.get 6
        i32.const 128
        i32.add
        i32.const 16
        i32.add
        i64.load
        i64.store
        local.get 6
        i32.const 200
        i32.add
        local.tee 8
        local.get 6
        i32.const 128
        i32.add
        i32.const 8
        i32.add
        i64.load
        i64.store
        local.get 6
        i32.const 160
        i32.add
        i32.const 8
        i32.add
        local.tee 7
        local.get 1
        i32.const 8
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 160
        i32.add
        i32.const 16
        i32.add
        local.tee 9
        local.get 1
        i32.const 16
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 160
        i32.add
        i32.const 24
        i32.add
        local.get 1
        i32.const 24
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        local.get 6
        i64.load offset=128
        i64.store offset=192
        local.get 6
        local.get 1
        i64.load align=1
        i64.store offset=160
        local.get 6
        i32.const 64
        i32.add
        local.get 6
        i32.const 160
        i32.add
        i32.const 64
        call $memcpy
        drop
        local.get 6
        i32.const 192
        i32.add
        local.get 4
        i64.store
        local.get 6
        i32.const 312
        i32.add
        local.get 2
        i32.const 24
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 304
        i32.add
        local.get 2
        i32.const 16
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 296
        i32.add
        local.get 2
        i32.const 8
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        local.get 3
        i64.store offset=184
        local.get 6
        local.get 2
        i64.load align=1
        i64.store offset=288
        local.get 8
        local.get 6
        i32.const 64
        i32.add
        i32.const 64
        call $memcpy
        local.set 8
        local.get 6
        i32.const 280
        i32.add
        local.tee 13
        i32.const -1443780867
        i32.store
        local.get 7
        i64.const 0
        i64.store
        local.get 9
        i64.const 0
        i64.store
        local.get 6
        i32.const 272
        i32.add
        local.tee 9
        local.get 5
        i32.const 8
        i32.add
        i32.load
        i32.store
        local.get 6
        i32.const 264
        i32.add
        local.tee 14
        local.get 5
        i64.load align=4
        i64.store
        local.get 6
        i64.const 0
        i64.store offset=160
        local.get 6
        i32.const 320
        i32.add
        i32.const 8
        i32.add
        local.tee 5
        i32.const 16384
        i32.store
        local.get 6
        i32.const 68528
        i32.store offset=324
        local.get 6
        i32.const 0
        i32.store offset=320
        local.get 6
        i32.const 40
        i32.add
        local.get 6
        i32.const 320
        i32.add
        local.get 6
        i32.const 288
        i32.add
        call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h57c9f8b1d4f26e96E
        local.get 6
        i32.load offset=44
        local.set 15
        local.get 6
        i32.load offset=40
        local.set 16
        local.get 6
        i32.const 32
        i32.add
        local.get 6
        i32.const 320
        i32.add
        local.get 7
        call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17hd2d0c7cb8dc086fdE
        local.get 6
        i32.load offset=36
        local.set 17
        local.get 6
        i32.load offset=32
        local.set 18
        local.get 6
        i64.load offset=324 align=4
        local.set 19
        local.get 6
        i32.const 0
        i32.store offset=136
        local.get 6
        local.get 19
        i64.store offset=128
        local.get 6
        i32.const 128
        i32.add
        local.get 13
        i32.const 4
        call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E
        local.get 6
        i32.const 232
        i32.add
        local.get 6
        i32.const 128
        i32.add
        call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E
        local.get 8
        local.get 6
        i32.const 128
        i32.add
        call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Encode$u20$for$u20$ink_env..types..AccountId$GT$9encode_to17h32fa861272bb3361E
        local.get 3
        local.get 4
        local.get 6
        i32.const 128
        i32.add
        call $_ZN18parity_scale_codec5codec6Encode9encode_to17hc5f3c4e04a903aa0E
        local.get 14
        i32.load
        local.get 9
        i32.load
        local.get 6
        i32.const 128
        i32.add
        call $_ZN65_$LT$$u5b$T$u5d$$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h61469f03473da43cE
        local.get 5
        i32.const 0
        i32.store
        local.get 6
        i32.const 68432
        i32.store offset=324
        local.get 6
        i32.load offset=132
        local.tee 8
        local.get 6
        i32.load offset=136
        local.tee 5
        i32.lt_u
        br_if 0 (;@2;)
        local.get 6
        i32.load offset=128
        local.set 7
        local.get 6
        local.get 8
        local.get 5
        i32.sub
        local.tee 8
        i32.store offset=340
        local.get 6
        local.get 7
        local.get 5
        i32.add
        local.tee 9
        i32.store offset=336
        local.get 6
        local.get 8
        i32.store offset=128
        local.get 16
        local.get 15
        i64.const 0
        local.get 18
        local.get 17
        local.get 7
        local.get 5
        local.get 9
        local.get 6
        i32.const 128
        i32.add
        call $_ZN7ink_env6engine8on_chain3ext3sys9seal_call17hcadb176a98e0eed8E
        local.set 8
        local.get 6
        i32.const 336
        i32.add
        local.get 6
        i32.load offset=128
        call $_ZN7ink_env6engine8on_chain3ext18extract_from_slice17h8e422b55d0a95429E
        i32.const 1
        local.set 7
        i32.const 1
        local.set 5
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      local.get 8
                                      call $_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h20933c84fa909027E
                                      i32.const -1
                                      i32.add
                                      br_table 13 (;@4;) 2 (;@15;) 3 (;@14;) 4 (;@13;) 5 (;@12;) 6 (;@11;) 7 (;@10;) 8 (;@9;) 9 (;@8;) 1 (;@16;) 10 (;@7;) 11 (;@6;) 0 (;@17;) 13 (;@4;)
                                    end
                                    local.get 6
                                    local.get 6
                                    i64.load offset=336
                                    i64.store offset=344
                                    local.get 6
                                    i32.const 24
                                    i32.add
                                    local.get 6
                                    i32.const 344
                                    i32.add
                                    call $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E
                                    local.get 6
                                    i32.load8_u offset=24
                                    i32.const 1
                                    i32.and
                                    br_if 11 (;@5;)
                                    i32.const 0
                                    local.set 8
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        local.get 6
                                        i32.load8_u offset=25
                                        i32.const 255
                                        i32.and
                                        br_table 1 (;@17;) 0 (;@18;) 13 (;@5;)
                                      end
                                      local.get 6
                                      i32.const 16
                                      i32.add
                                      local.get 6
                                      i32.const 344
                                      i32.add
                                      call $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E
                                      local.get 6
                                      i32.load8_u offset=16
                                      i32.const 1
                                      i32.and
                                      local.get 6
                                      i32.load8_u offset=17
                                      i32.or
                                      br_if 12 (;@5;)
                                      local.get 6
                                      i32.const 128
                                      i32.add
                                      local.get 6
                                      i32.const 344
                                      i32.add
                                      call $_ZN75_$LT$alloc..string..String$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4e2a95228ac42f1aE
                                      local.get 6
                                      i32.load offset=128
                                      local.tee 8
                                      i32.eqz
                                      br_if 12 (;@5;)
                                      local.get 6
                                      i64.load offset=132 align=4
                                      local.set 19
                                    end
                                    i32.const 0
                                    local.set 7
                                    br 13 (;@3;)
                                  end
                                  unreachable
                                end
                                i32.const 2
                                local.set 5
                                br 10 (;@4;)
                              end
                              i32.const 3
                              local.set 5
                              br 9 (;@4;)
                            end
                            i32.const 4
                            local.set 5
                            br 8 (;@4;)
                          end
                          i32.const 5
                          local.set 5
                          br 7 (;@4;)
                        end
                        i32.const 6
                        local.set 5
                        br 6 (;@4;)
                      end
                      i32.const 7
                      local.set 5
                      br 5 (;@4;)
                    end
                    i32.const 8
                    local.set 5
                    br 4 (;@4;)
                  end
                  i32.const 10
                  local.set 5
                  br 3 (;@4;)
                end
                i32.const 11
                local.set 5
                br 2 (;@4;)
              end
              i32.const 9
              local.set 5
              br 1 (;@4;)
            end
            i32.const 0
            local.set 5
          end
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 7
            br_if 0 (;@4;)
            local.get 8
            i32.eqz
            br_if 1 (;@3;)
            local.get 6
            local.get 19
            i64.store offset=132 align=4
            local.get 6
            local.get 8
            i32.store offset=128
            local.get 6
            i32.const 180
            i32.add
            i32.const 1
            i32.store
            local.get 6
            i64.const 1
            i64.store offset=164 align=4
            local.get 6
            i32.const 65632
            i32.store offset=160
            local.get 6
            i32.const 1
            i32.store offset=324
            local.get 6
            local.get 6
            i32.const 320
            i32.add
            i32.store offset=176
            local.get 6
            local.get 6
            i32.const 128
            i32.add
            i32.store offset=320
            local.get 6
            i32.const 64
            i32.add
            local.get 6
            i32.const 160
            i32.add
            call $_ZN5alloc3fmt6format17hb056d6bf77a19e8dE
            br 2 (;@2;)
          end
          local.get 5
          i32.const 255
          i32.and
          i32.const 8
          i32.eq
          br_if 0 (;@3;)
          local.get 6
          local.get 5
          i32.store8 offset=320
          local.get 6
          i32.const 180
          i32.add
          i32.const 1
          i32.store
          local.get 6
          i64.const 1
          i64.store offset=164 align=4
          local.get 6
          i32.const 65568
          i32.store offset=160
          local.get 6
          i32.const 2
          i32.store offset=132
          local.get 6
          local.get 6
          i32.const 128
          i32.add
          i32.store offset=176
          local.get 6
          local.get 6
          i32.const 320
          i32.add
          i32.store offset=128
          local.get 6
          i32.const 64
          i32.add
          local.get 6
          i32.const 160
          i32.add
          call $_ZN5alloc3fmt6format17hb056d6bf77a19e8dE
          br 1 (;@2;)
        end
        local.get 6
        i32.const 160
        i32.add
        i32.const 24
        i32.add
        local.tee 5
        local.get 1
        i32.const 24
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 160
        i32.add
        i32.const 16
        i32.add
        local.tee 7
        local.get 1
        i32.const 16
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        i32.const 160
        i32.add
        i32.const 8
        i32.add
        local.tee 8
        local.get 1
        i32.const 8
        i32.add
        i64.load align=1
        i64.store
        local.get 6
        local.get 1
        i64.load align=1
        i64.store offset=160
        local.get 0
        i32.const 72
        i32.add
        local.tee 14
        local.get 6
        i32.const 160
        i32.add
        local.get 10
        local.get 3
        i64.sub
        local.get 12
        local.get 4
        i64.sub
        local.get 11
        i64.extend_i32_u
        i64.sub
        call $_ZN11ink_storage11collections7hashmap24HashMap$LT$K$C$V$C$H$GT$6insert17hd1bfc3a3e6a0334aE
        local.get 5
        local.get 2
        i32.const 24
        i32.add
        local.tee 1
        i64.load align=1
        i64.store
        local.get 7
        local.get 2
        i32.const 16
        i32.add
        local.tee 9
        i64.load align=1
        i64.store
        local.get 8
        local.get 2
        i32.const 8
        i32.add
        local.tee 13
        i64.load align=1
        i64.store
        local.get 6
        local.get 2
        i64.load align=1
        i64.store offset=160
        local.get 6
        local.get 0
        local.get 6
        i32.const 160
        i32.add
        call $_ZN5psp226traits5PSP2210balance_of17h8e8fb26b27704ba9E
        local.get 6
        i32.const 8
        i32.add
        i64.load
        local.set 12
        local.get 6
        i64.load
        local.set 19
        local.get 5
        local.get 1
        i64.load align=1
        i64.store
        local.get 7
        local.get 9
        i64.load align=1
        i64.store
        local.get 8
        local.get 13
        i64.load align=1
        i64.store
        local.get 6
        local.get 2
        i64.load align=1
        i64.store offset=160
        local.get 19
        local.get 3
        i64.add
        local.tee 3
        local.get 19
        i64.lt_u
        local.tee 2
        local.get 12
        local.get 4
        i64.add
        local.get 2
        i64.extend_i32_u
        i64.add
        local.tee 4
        local.get 12
        i64.lt_u
        local.get 4
        local.get 12
        i64.eq
        select
        i32.eqz
        br_if 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 14
    local.get 6
    i32.const 160
    i32.add
    local.get 3
    local.get 4
    call $_ZN11ink_storage11collections7hashmap24HashMap$LT$K$C$V$C$H$GT$6insert17hd1bfc3a3e6a0334aE
    local.get 6
    i32.const 352
    i32.add
    global.set 0)
  (func $_ZN69_$LT$T$u20$as$u20$core..array..equality..SpecArrayEq$LT$U$C$_$GT$$GT$7spec_eq17heb72ec737bd773c4E (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.const 32
    call $memcmp
    i32.eqz)
  (func $_ZN76_$LT$alloc..string..String$u20$as$u20$core..convert..From$LT$$RF$str$GT$$GT$4from17hefa3ad3efa3f0b3eE (type 5) (param i32)
    local.get 0
    i32.const 65680
    i32.const 20
    call $_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9to_vec_in17ha74b5d7f675ed47eE)
  (func $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE (type 5) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 16384
    i32.store offset=44
    local.get 1
    i32.const 68528
    i32.store offset=40
    local.get 1
    i32.const 16384
    i32.store offset=48
    i32.const 68528
    local.get 1
    i32.const 48
    i32.add
    call $_ZN7ink_env6engine8on_chain3ext3sys11seal_caller17hf527ac368aa51f8aE
    local.get 1
    i32.const 40
    i32.add
    local.get 1
    i32.load offset=48
    call $_ZN7ink_env6engine8on_chain3ext18extract_from_slice17h8e422b55d0a95429E
    local.get 1
    local.get 1
    i64.load offset=40
    i64.store offset=88
    local.get 1
    i32.const 48
    i32.add
    local.get 1
    i32.const 88
    i32.add
    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
    i32.const 0
    local.set 2
    block  ;; label = @1
      local.get 1
      i32.load8_u offset=48
      local.tee 3
      i32.const 1
      i32.eq
      br_if 0 (;@1;)
      local.get 1
      i32.const 16
      i32.add
      local.get 1
      i32.const 58
      i32.add
      i64.load align=2
      i64.store
      local.get 1
      i32.const 24
      i32.add
      local.get 1
      i32.const 66
      i32.add
      i64.load align=2
      i64.store
      local.get 1
      i32.const 31
      i32.add
      local.get 1
      i32.const 73
      i32.add
      i64.load align=1
      i64.store align=1
      local.get 1
      local.get 1
      i64.load offset=50 align=2
      i64.store offset=8
      local.get 1
      i32.load8_u offset=49
      local.set 2
    end
    block  ;; label = @1
      local.get 3
      i32.const 1
      i32.ne
      br_if 0 (;@1;)
      call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
      unreachable
    end
    local.get 0
    local.get 2
    i32.store8
    local.get 0
    local.get 1
    i64.load offset=8
    i64.store offset=1 align=1
    local.get 0
    i32.const 9
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load
    i64.store align=1
    local.get 0
    i32.const 17
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load
    i64.store align=1
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i32.const 31
    i32.add
    i64.load align=1
    i64.store align=1
    local.get 1
    i32.const 96
    i32.add
    global.set 0)
  (func $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E (type 2) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=8
        local.tee 4
        local.get 2
        i32.add
        local.tee 5
        local.get 4
        i32.lt_u
        br_if 0 (;@2;)
        local.get 3
        i32.const 8
        i32.add
        local.get 4
        local.get 5
        local.get 0
        i32.load
        local.get 0
        i32.load offset=4
        call $_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$9index_mut17h6d040d6921c9ef33E
        local.get 3
        i32.load offset=12
        local.get 2
        i32.ne
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=8
        local.get 1
        local.get 2
        call $memcpy
        drop
        local.get 0
        local.get 5
        i32.store offset=8
        local.get 3
        i32.const 16
        i32.add
        global.set 0
        return
      end
      unreachable
      unreachable
    end
    call $_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17hca07929dff42a255E
    unreachable)
  (func $_ZN18parity_scale_codec5codec6Encode9encode_to17hc5f3c4e04a903aa0E (type 12) (param i64 i64 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    local.get 1
    i64.store offset=8
    local.get 3
    local.get 0
    i64.store
    local.get 2
    local.get 3
    i32.const 16
    call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN65_$LT$$u5b$T$u5d$$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h61469f03473da43cE (type 2) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 63
        i32.gt_u
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.const 2
        i32.shl
        call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$9push_byte17hcf753f1cf9d2ac5bE
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        i32.const 16383
        i32.gt_u
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.const 2
        i32.shl
        i32.const 1
        i32.or
        i32.store16 offset=14
        local.get 2
        local.get 3
        i32.const 14
        i32.add
        i32.const 2
        call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E
        br 1 (;@1;)
      end
      block  ;; label = @2
        local.get 1
        i32.const 1073741823
        i32.gt_u
        br_if 0 (;@2;)
        local.get 1
        i32.const 2
        i32.shl
        i32.const 2
        i32.or
        local.get 2
        call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E
        br 1 (;@1;)
      end
      local.get 2
      i32.const 3
      call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$9push_byte17hcf753f1cf9d2ac5bE
      local.get 1
      local.get 2
      call $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E
    end
    local.get 2
    local.get 0
    local.get 1
    call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN7ink_env6engine8on_chain3ext18extract_from_slice17h8e422b55d0a95429E (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 0
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call $_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$9index_mut17h6d040d6921c9ef33E
    local.get 0
    local.get 2
    i64.load offset=8
    i64.store align=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN50_$LT$T$u20$as$u20$core..convert..Into$LT$U$GT$$GT$4into17h20933c84fa909027E (type 7) (param i32) (result i32)
    (local i32)
    i32.const 12
    local.set 1
    block  ;; label = @1
      local.get 0
      i32.const 11
      i32.gt_u
      br_if 0 (;@1;)
      local.get 0
      i32.const 2
      i32.shl
      i32.const 68472
      i32.add
      i32.load
      local.set 1
    end
    local.get 1)
  (func $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E (type 4) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store8 offset=15
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.const 15
        i32.add
        i32.const 1
        call $_ZN69_$LT$$RF$$u5b$u8$u5d$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17ha43a819276410a89E
        local.tee 1
        br_if 0 (;@2;)
        local.get 2
        i32.load8_u offset=15
        local.set 3
        br 1 (;@1;)
      end
    end
    local.get 0
    local.get 3
    i32.store8 offset=1
    local.get 0
    local.get 1
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN70_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb6cda997336d4a2dE (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load
        br_if 0 (;@2;)
        local.get 2
        local.get 1
        i32.load offset=24
        i32.const 65660
        i32.const 2
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        i32.store8 offset=8
        local.get 2
        local.get 1
        i32.store
        local.get 2
        i32.const 0
        i32.store8 offset=9
        local.get 2
        i32.const 0
        i32.store offset=4
        local.get 2
        local.get 0
        i32.store offset=12
        local.get 2
        local.get 2
        i32.const 12
        i32.add
        i32.const 65664
        call $_ZN4core3fmt8builders10DebugTuple5field17hfbb7b61041be766dE
        call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.load offset=24
      i32.const 65640
      i32.const 3
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      i32.store8 offset=8
      local.get 2
      local.get 1
      i32.store
      local.get 2
      i32.const 0
      i32.store8 offset=9
      local.get 2
      i32.const 0
      i32.store offset=4
      local.get 2
      local.get 0
      i32.store offset=12
      local.get 2
      local.get 2
      i32.const 12
      i32.add
      i32.const 65644
      call $_ZN4core3fmt8builders10DebugTuple5field17hfbb7b61041be766dE
      call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
      local.set 1
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN5alloc3fmt6format17hb056d6bf77a19e8dE (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 1
    i32.load offset=4
    local.tee 3
    i32.const 3
    i32.shl
    local.set 4
    i32.const 0
    local.set 5
    local.get 1
    i32.load
    local.tee 6
    local.set 7
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            local.get 4
            i32.eqz
            br_if 1 (;@3;)
            local.get 5
            local.get 7
            i32.load offset=4
            i32.add
            local.tee 8
            local.get 5
            i32.lt_u
            br_if 2 (;@2;)
            local.get 4
            i32.const -8
            i32.add
            local.set 4
            local.get 7
            i32.const 8
            i32.add
            local.set 7
            local.get 8
            local.set 5
            br 0 (;@4;)
          end
        end
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 20
            i32.add
            i32.load
            br_if 0 (;@4;)
            local.get 5
            local.set 4
            br 1 (;@3;)
          end
          block  ;; label = @4
            local.get 3
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.load offset=4
            br_if 0 (;@4;)
            i32.const 0
            local.set 4
            local.get 5
            i32.const 16
            i32.lt_u
            br_if 1 (;@3;)
          end
          i32.const 0
          local.get 5
          local.get 5
          i32.add
          local.tee 4
          local.get 4
          local.get 5
          i32.lt_u
          select
          local.set 4
        end
        local.get 2
        local.get 4
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16with_capacity_in17h55f2c99e227cc199E
        local.get 2
        i64.load
        local.set 9
        local.get 0
        i32.const 0
        i32.store offset=8
        local.get 0
        local.get 9
        i64.store align=4
        local.get 2
        i32.const 8
        i32.add
        i32.const 16
        i32.add
        local.get 1
        i32.const 16
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        i32.const 8
        i32.add
        i32.const 8
        i32.add
        local.get 1
        i32.const 8
        i32.add
        i64.load align=4
        i64.store
        local.get 2
        local.get 1
        i64.load align=4
        i64.store offset=8
        local.get 0
        local.get 2
        i32.const 8
        i32.add
        call $_ZN4core3fmt5Write9write_fmt17h147d222b45c4182cE
        br_if 1 (;@1;)
        local.get 2
        i32.const 32
        i32.add
        global.set 0
        return
      end
      unreachable
      unreachable
    end
    call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
    unreachable)
  (func $_ZN58_$LT$ink_env..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h47fa7ba2ded42968E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              local.get 0
                              i32.load8_u
                              br_table 0 (;@13;) 1 (;@12;) 2 (;@11;) 3 (;@10;) 4 (;@9;) 5 (;@8;) 6 (;@7;) 7 (;@6;) 8 (;@5;) 9 (;@4;) 10 (;@3;) 11 (;@2;) 0 (;@13;)
                            end
                            local.get 2
                            local.get 1
                            i32.load offset=24
                            i32.const 68409
                            i32.const 6
                            local.get 1
                            i32.const 28
                            i32.add
                            i32.load
                            i32.load offset=12
                            call_indirect (type 0)
                            i32.store8 offset=24
                            local.get 2
                            local.get 1
                            i32.store offset=16
                            local.get 2
                            i32.const 0
                            i32.store8 offset=25
                            local.get 2
                            i32.const 0
                            i32.store offset=20
                            local.get 2
                            local.get 0
                            i32.const 1
                            i32.add
                            i32.store offset=12
                            local.get 2
                            i32.const 16
                            i32.add
                            local.get 2
                            i32.const 12
                            i32.add
                            i32.const 68416
                            call $_ZN4core3fmt8builders10DebugTuple5field17hfbb7b61041be766dE
                            call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
                            local.set 1
                            br 11 (;@1;)
                          end
                          local.get 2
                          local.get 1
                          i32.load offset=24
                          i32.const 68396
                          i32.const 13
                          local.get 1
                          i32.const 28
                          i32.add
                          i32.load
                          i32.load offset=12
                          call_indirect (type 0)
                          i32.store8 offset=24
                          local.get 2
                          local.get 1
                          i32.store offset=16
                          local.get 2
                          i32.const 0
                          i32.store8 offset=25
                          local.get 2
                          i32.const 0
                          i32.store offset=20
                          local.get 2
                          i32.const 16
                          i32.add
                          call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
                          local.set 1
                          br 10 (;@1;)
                        end
                        local.get 2
                        local.get 1
                        i32.load offset=24
                        i32.const 68382
                        i32.const 14
                        local.get 1
                        i32.const 28
                        i32.add
                        i32.load
                        i32.load offset=12
                        call_indirect (type 0)
                        i32.store8 offset=24
                        local.get 2
                        local.get 1
                        i32.store offset=16
                        local.get 2
                        i32.const 0
                        i32.store8 offset=25
                        local.get 2
                        i32.const 0
                        i32.store offset=20
                        local.get 2
                        i32.const 16
                        i32.add
                        call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
                        local.set 1
                        br 9 (;@1;)
                      end
                      local.get 2
                      local.get 1
                      i32.load offset=24
                      i32.const 68371
                      i32.const 11
                      local.get 1
                      i32.const 28
                      i32.add
                      i32.load
                      i32.load offset=12
                      call_indirect (type 0)
                      i32.store8 offset=24
                      local.get 2
                      local.get 1
                      i32.store offset=16
                      local.get 2
                      i32.const 0
                      i32.store8 offset=25
                      local.get 2
                      i32.const 0
                      i32.store offset=20
                      local.get 2
                      i32.const 16
                      i32.add
                      call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
                      local.set 1
                      br 8 (;@1;)
                    end
                    local.get 2
                    local.get 1
                    i32.load offset=24
                    i32.const 68346
                    i32.const 25
                    local.get 1
                    i32.const 28
                    i32.add
                    i32.load
                    i32.load offset=12
                    call_indirect (type 0)
                    i32.store8 offset=24
                    local.get 2
                    local.get 1
                    i32.store offset=16
                    local.get 2
                    i32.const 0
                    i32.store8 offset=25
                    local.get 2
                    i32.const 0
                    i32.store offset=20
                    local.get 2
                    i32.const 16
                    i32.add
                    call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
                    local.set 1
                    br 7 (;@1;)
                  end
                  local.get 2
                  local.get 1
                  i32.load offset=24
                  i32.const 68332
                  i32.const 14
                  local.get 1
                  i32.const 28
                  i32.add
                  i32.load
                  i32.load offset=12
                  call_indirect (type 0)
                  i32.store8 offset=24
                  local.get 2
                  local.get 1
                  i32.store offset=16
                  local.get 2
                  i32.const 0
                  i32.store8 offset=25
                  local.get 2
                  i32.const 0
                  i32.store offset=20
                  local.get 2
                  i32.const 16
                  i32.add
                  call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
                  local.set 1
                  br 6 (;@1;)
                end
                local.get 2
                local.get 1
                i32.load offset=24
                i32.const 68312
                i32.const 20
                local.get 1
                i32.const 28
                i32.add
                i32.load
                i32.load offset=12
                call_indirect (type 0)
                i32.store8 offset=24
                local.get 2
                local.get 1
                i32.store offset=16
                local.get 2
                i32.const 0
                i32.store8 offset=25
                local.get 2
                i32.const 0
                i32.store offset=20
                local.get 2
                i32.const 16
                i32.add
                call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
                local.set 1
                br 5 (;@1;)
              end
              local.get 2
              local.get 1
              i32.load offset=24
              i32.const 68300
              i32.const 12
              local.get 1
              i32.const 28
              i32.add
              i32.load
              i32.load offset=12
              call_indirect (type 0)
              i32.store8 offset=24
              local.get 2
              local.get 1
              i32.store offset=16
              local.get 2
              i32.const 0
              i32.store8 offset=25
              local.get 2
              i32.const 0
              i32.store offset=20
              local.get 2
              i32.const 16
              i32.add
              call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
              local.set 1
              br 4 (;@1;)
            end
            local.get 2
            local.get 1
            i32.load offset=24
            i32.const 68289
            i32.const 11
            local.get 1
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 0)
            i32.store8 offset=24
            local.get 2
            local.get 1
            i32.store offset=16
            local.get 2
            i32.const 0
            i32.store8 offset=25
            local.get 2
            i32.const 0
            i32.store offset=20
            local.get 2
            i32.const 16
            i32.add
            call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.load offset=24
          i32.const 68282
          i32.const 7
          local.get 1
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          i32.store8 offset=24
          local.get 2
          local.get 1
          i32.store offset=16
          local.get 2
          i32.const 0
          i32.store8 offset=25
          local.get 2
          i32.const 0
          i32.store offset=20
          local.get 2
          i32.const 16
          i32.add
          call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.load offset=24
        i32.const 68267
        i32.const 15
        local.get 1
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        i32.store8 offset=24
        local.get 2
        local.get 1
        i32.store offset=16
        local.get 2
        i32.const 0
        i32.store8 offset=25
        local.get 2
        i32.const 0
        i32.store offset=20
        local.get 2
        i32.const 16
        i32.add
        call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.load offset=24
      i32.const 68249
      i32.const 18
      local.get 1
      i32.const 28
      i32.add
      i32.load
      i32.load offset=12
      call_indirect (type 0)
      i32.store8 offset=24
      local.get 2
      local.get 1
      i32.store offset=16
      local.get 2
      i32.const 0
      i32.store8 offset=25
      local.get 2
      i32.const 0
      i32.store offset=20
      local.get 2
      i32.const 16
      i32.add
      call $_ZN4core3fmt8builders10DebugTuple6finish17hf3171f1700c4507aE
      local.set 1
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN5psp226traits5PSP229allowance17h133c5b43bbd89b5bE (type 13) (param i32 i32 i32 i32)
    (local i32 i64 i64)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    i32.const 24
    i32.add
    local.get 2
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    i32.const 16
    i32.add
    local.get 2
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    i32.const 40
    i32.add
    local.get 3
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    i32.const 48
    i32.add
    local.get 3
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    i32.const 56
    i32.add
    local.get 3
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    local.get 2
    i64.load align=1
    i64.store
    local.get 4
    local.get 3
    i64.load align=1
    i64.store offset=32
    i64.const 0
    local.set 5
    i64.const 0
    local.set 6
    block  ;; label = @1
      local.get 1
      i32.const 352
      i32.add
      local.get 4
      call $_ZN11ink_storage4lazy9lazy_hmap28LazyHashMap$LT$K$C$V$C$H$GT$11lazily_load17ha761820488558d57E
      local.tee 3
      i64.load
      i64.const 1
      i64.ne
      br_if 0 (;@1;)
      local.get 3
      i32.const 16
      i32.add
      i64.load
      local.set 6
      local.get 3
      i64.load offset=8
      local.set 5
    end
    local.get 0
    local.get 5
    i64.store
    local.get 0
    local.get 6
    i64.store offset=8
    local.get 4
    i32.const 64
    i32.add
    global.set 0)
  (func $_ZN68_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17hd2312dbe62848addE (type 4) (param i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 1
      i32.load
      local.tee 2
      br_if 0 (;@1;)
      local.get 0
      i32.const 0
      i32.store
      return
    end
    local.get 0
    local.get 2
    local.get 1
    i32.const 8
    i32.add
    i32.load
    call $_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9to_vec_in17ha74b5d7f675ed47eE)
  (func $_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$9to_vec_in17ha74b5d7f675ed47eE (type 2) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 2
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16with_capacity_in17h55f2c99e227cc199E
    local.get 3
    i32.load offset=8
    local.set 4
    local.get 0
    local.get 3
    i32.load offset=12
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store
    local.get 4
    local.get 1
    local.get 2
    call $memcpy
    drop
    local.get 0
    local.get 2
    i32.store offset=8
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN75_$LT$ink_storage..lazy..Lazy$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h8682d878e121a73fE (type 5) (param i32)
    local.get 0
    i64.const 1
    i64.store offset=40
    local.get 0
    i64.const 0
    i64.store
    local.get 0
    i32.const 56
    i32.add
    i32.const 0
    i32.store8)
  (func $_ZN76_$LT$$u5b$T$u3b$$u20$N$u5d$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h6fb3ac2ce221d964E (type 14) (param i32) (result i64)
    (local i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i64.const 0
    i64.store offset=8
    i32.const 4
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.const 8
            i32.ne
            br_if 0 (;@4;)
            local.get 1
            i32.const 4
            i32.store offset=8
            local.get 1
            i64.load offset=8
            local.tee 3
            i32.wrap_i64
            i32.const 4
            i32.ge_u
            br_if 2 (;@2;)
            unreachable
            unreachable
          end
          local.get 1
          local.get 0
          call $_ZN56_$LT$u8$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h374e211fca1804deE
          block  ;; label = @4
            local.get 1
            i32.load8_u
            i32.const 1
            i32.and
            br_if 0 (;@4;)
            local.get 1
            i32.const 8
            i32.add
            local.get 2
            i32.add
            local.get 1
            i32.load8_u offset=1
            i32.store8
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            br 1 (;@3;)
          end
        end
        i32.const 1
        local.set 2
        i32.const 0
        local.set 0
        br 1 (;@1;)
      end
      local.get 3
      i64.const 32
      i64.shr_u
      i32.wrap_i64
      local.set 0
      i32.const 0
      local.set 2
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0
    local.get 0
    i64.extend_i32_u
    i64.const 8
    i64.shl
    local.get 2
    i64.extend_i32_u
    i64.or)
  (func $_ZN56_$LT$u8$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h374e211fca1804deE (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E
    local.get 2
    i32.load8_u offset=8
    local.set 1
    local.get 0
    local.get 2
    i32.load8_u offset=9
    i32.store8 offset=1
    local.get 0
    local.get 1
    i32.const 1
    i32.and
    i32.store8
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN7ink_env3api12return_value17h0a13252c3ce29282E (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    call $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17h0acede7be64717ccE
    unreachable)
  (func $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17h0acede7be64717ccE (type 5) (param i32)
    local.get 0
    call $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17hb79762755f1c41deE
    unreachable)
  (func $_ZN7ink_env3api12return_value17h30fd262fc41c2a34E (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    call $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17hdd171eae8ad46563E
    unreachable)
  (func $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17hdd171eae8ad46563E (type 5) (param i32)
    local.get 0
    call $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17hf40320b83a76f1d2E
    unreachable)
  (func $_ZN7ink_env3api12return_value17h7467eb0b235c2036E (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    call $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17h19d1b67e32fc8062E
    unreachable)
  (func $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17h19d1b67e32fc8062E (type 5) (param i32)
    local.get 0
    call $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17hea4418b9b856770cE
    unreachable)
  (func $_ZN7ink_env3api12return_value17he3784c752d74a359E (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.store offset=12
    local.get 1
    i32.const 12
    i32.add
    call $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17h991168ec5b6fabcdE
    unreachable)
  (func $_ZN86_$LT$ink_env..engine..on_chain..EnvInstance$u20$as$u20$ink_env..engine..OnInstance$GT$11on_instance17h991168ec5b6fabcdE (type 5) (param i32)
    local.get 0
    call $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17h5d6850e231deab58E
    unreachable)
  (func $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17h5d6850e231deab58E (type 5) (param i32)
    local.get 0
    i32.load
    call $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17hbb159bbf92296044E
    unreachable)
  (func $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17hbb159bbf92296044E (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 24
    i32.add
    i32.const 16384
    i32.store
    local.get 1
    i32.const 68528
    i32.store offset=20
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h20963af99cf5deb7E
    local.get 1
    i32.load offset=8
    local.get 1
    i32.load offset=12
    call $_ZN7ink_env6engine8on_chain3ext12return_value17hec70034dcc72bf9aE
    unreachable)
  (func $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17hb79762755f1c41deE (type 5) (param i32)
    local.get 0
    i32.load
    call $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17h1f0a6d3d46880e3eE
    unreachable)
  (func $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17h1f0a6d3d46880e3eE (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 24
    i32.add
    i32.const 16384
    i32.store
    local.get 1
    i32.const 68528
    i32.store offset=20
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h57c9f8b1d4f26e96E
    local.get 1
    i32.load offset=8
    local.get 1
    i32.load offset=12
    call $_ZN7ink_env6engine8on_chain3ext12return_value17hec70034dcc72bf9aE
    unreachable)
  (func $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17hea4418b9b856770cE (type 5) (param i32)
    local.get 0
    i32.load
    call $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17h5b9981aa7950b66bE
    unreachable)
  (func $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17h5b9981aa7950b66bE (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 24
    i32.add
    i32.const 16384
    i32.store
    local.get 1
    i32.const 68528
    i32.store offset=20
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h3d76e12c885d5f42E
    local.get 1
    i32.load offset=8
    local.get 1
    i32.load offset=12
    call $_ZN7ink_env6engine8on_chain3ext12return_value17hec70034dcc72bf9aE
    unreachable)
  (func $_ZN7ink_env3api12return_value28_$u7b$$u7b$closure$u7d$$u7d$17hf40320b83a76f1d2E (type 5) (param i32)
    local.get 0
    i32.load
    call $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17hc0d495cf6804f9e8E
    unreachable)
  (func $_ZN7ink_env6engine8on_chain5impls97_$LT$impl$u20$ink_env..backend..EnvBackend$u20$for$u20$ink_env..engine..on_chain..EnvInstance$GT$12return_value17hc0d495cf6804f9e8E (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 24
    i32.add
    i32.const 16384
    i32.store
    local.get 1
    i32.const 68528
    i32.store offset=20
    local.get 1
    i32.const 0
    i32.store offset=16
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    local.get 0
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17hd2d0c7cb8dc086fdE
    local.get 1
    i32.load offset=8
    local.get 1
    i32.load offset=12
    call $_ZN7ink_env6engine8on_chain3ext12return_value17hec70034dcc72bf9aE
    unreachable)
  (func $_ZN7ink_env6engine8on_chain3ext12return_value17hec70034dcc72bf9aE (type 4) (param i32 i32)
    i32.const 0
    local.get 0
    local.get 1
    call $_ZN7ink_env6engine8on_chain3ext3sys11seal_return17h1344b67397975fd3E
    unreachable)
  (func $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h3d76e12c885d5f42E (type 2) (param i32 i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    local.tee 4
    i32.load
    local.set 5
    local.get 4
    i32.const 0
    i32.store
    local.get 1
    i32.load offset=4
    local.set 4
    local.get 1
    i32.const 68432
    i32.store offset=4
    local.get 3
    local.get 5
    i32.store offset=20
    local.get 3
    local.get 4
    i32.store offset=16
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load
          br_if 0 (;@3;)
          local.get 5
          i32.eqz
          br_if 2 (;@1;)
          local.get 4
          i32.const 0
          i32.store8
          i32.const 1
          local.set 2
          br 1 (;@2;)
        end
        local.get 5
        i32.eqz
        br_if 1 (;@1;)
        local.get 4
        i32.const 1
        i32.store8
        local.get 3
        i32.const 1
        i32.store offset=24
        local.get 2
        local.get 3
        i32.const 16
        i32.add
        call $_ZN55_$LT$X$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h8e4e3faea8e8d6f3E
        local.get 3
        i32.load offset=20
        local.set 5
        local.get 3
        i32.load offset=16
        local.set 4
        local.get 3
        i32.load offset=24
        local.set 2
      end
      local.get 1
      local.get 5
      i32.store offset=8
      local.get 1
      local.get 4
      i32.store offset=4
      local.get 3
      i32.const 8
      i32.add
      local.get 1
      local.get 2
      call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer4take17hd78259512cf0470aE
      local.get 0
      local.get 3
      i64.load offset=8
      i64.store
      local.get 3
      i32.const 32
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer12take_encoded17h20963af99cf5deb7E (type 2) (param i32 i32 i32)
    (local i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i64.load offset=4 align=4
    local.set 4
    local.get 3
    i32.const 0
    i32.store offset=24
    local.get 3
    local.get 4
    i64.store offset=16
    local.get 3
    local.get 2
    i32.load8_u
    i32.store8 offset=31
    local.get 3
    i32.const 16
    i32.add
    local.get 3
    i32.const 31
    i32.add
    i32.const 1
    call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E
    local.get 1
    local.get 3
    i64.load offset=16
    i64.store offset=4 align=4
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 3
    i32.load offset=24
    call $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer4take17hd78259512cf0470aE
    local.get 0
    local.get 3
    i64.load offset=8
    i64.store
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func $_ZN55_$LT$X$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h8e4e3faea8e8d6f3E (type 4) (param i32 i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    local.get 1
    call $_ZN65_$LT$$u5b$T$u5d$$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h61469f03473da43cE)
  (func $_ZN7ink_env6engine8on_chain6buffer12ScopedBuffer4take17hd78259512cf0470aE (type 2) (param i32 i32 i32)
    (local i32 i32)
    local.get 1
    i32.const 8
    i32.add
    local.tee 3
    i32.load
    local.set 4
    local.get 3
    i32.const 0
    i32.store
    local.get 1
    i32.load offset=4
    local.set 3
    local.get 1
    i32.const 68432
    i32.store offset=4
    block  ;; label = @1
      local.get 4
      local.get 2
      i32.ge_u
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 1
    local.get 4
    local.get 2
    i32.sub
    i32.store offset=8
    local.get 1
    local.get 3
    local.get 2
    i32.add
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store)
  (func $_ZN83_$LT$core..option..Option$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17he6b92fd8601f68d9E (type 4) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.load8_u offset=8
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.load8_u offset=9
          i32.const 255
          i32.and
          br_table 0 (;@3;) 1 (;@2;) 2 (;@1;)
        end
        i32.const 0
        local.set 3
        local.get 0
        i32.const 0
        i32.store offset=4
        br 1 (;@1;)
      end
      local.get 2
      i32.const 16
      i32.add
      local.get 1
      call $_ZN75_$LT$alloc..string..String$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4e2a95228ac42f1aE
      local.get 2
      i32.load offset=16
      local.tee 1
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      i32.const 8
      i32.add
      local.get 2
      i64.load offset=20 align=4
      i64.store align=4
      local.get 0
      local.get 1
      i32.store offset=4
      i32.const 0
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func $deploy (type 15) (result i32)
    i32.const 0
    call $_ZN8my_psp228my_psp221_95_$LT$impl$u20$ink_lang..contract..DispatchUsingMode$u20$for$u20$my_psp22..my_psp22..MyPSP22$GT$19dispatch_using_mode17h80c60cbc72520667E
    i32.const 255
    i32.and
    i32.const 2
    i32.shl
    i32.const 65700
    i32.add
    i32.load)
  (func $_ZN8my_psp228my_psp221_95_$LT$impl$u20$ink_lang..contract..DispatchUsingMode$u20$for$u20$my_psp22..my_psp22..MyPSP22$GT$19dispatch_using_mode17h80c60cbc72520667E (type 7) (param i32) (result i32)
    (local i32 i64 i64 i32 i32 i32 i32 i32 i32 i32 i32 i64 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 1856
    i32.sub
    local.tee 1
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 0
                            br_if 0 (;@12;)
                            local.get 1
                            i32.const 16384
                            i32.store offset=348
                            local.get 1
                            i32.const 68528
                            i32.store offset=344
                            local.get 1
                            i32.const 344
                            i32.add
                            call $_ZN7ink_env6engine8on_chain3ext5input17h026d86c7dec708cfE
                            local.get 1
                            local.get 1
                            i64.load offset=344
                            i64.store offset=432
                            i32.const 1
                            local.set 0
                            local.get 1
                            i32.const 432
                            i32.add
                            call $_ZN76_$LT$$u5b$T$u3b$$u20$N$u5d$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h6fb3ac2ce221d964E
                            i64.const 1099511627521
                            i64.and
                            i64.const 406372391680
                            i64.ne
                            br_if 6 (;@6;)
                            local.get 1
                            i32.const 264
                            i32.add
                            local.get 1
                            i32.const 432
                            i32.add
                            call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
                            local.get 1
                            i32.load offset=264
                            br_if 6 (;@6;)
                            local.get 1
                            i32.const 280
                            i32.add
                            i64.load
                            local.set 2
                            local.get 1
                            i64.load offset=272
                            local.set 3
                            local.get 1
                            i32.const 1240
                            i32.add
                            local.get 1
                            i32.const 432
                            i32.add
                            call $_ZN83_$LT$core..option..Option$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17he6b92fd8601f68d9E
                            i32.const 1
                            local.set 0
                            local.get 1
                            i32.load offset=1240
                            i32.const 1
                            i32.eq
                            br_if 6 (;@6;)
                            local.get 1
                            i32.const 520
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 1
                            i32.const 1252
                            i32.add
                            local.tee 0
                            i32.load
                            i32.store
                            local.get 1
                            local.get 1
                            i64.load offset=1244 align=4
                            i64.store offset=520
                            local.get 1
                            i32.const 1240
                            i32.add
                            local.get 1
                            i32.const 432
                            i32.add
                            call $_ZN83_$LT$core..option..Option$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17he6b92fd8601f68d9E
                            local.get 1
                            i32.load offset=1240
                            i32.const 1
                            i32.eq
                            br_if 1 (;@11;)
                            local.get 1
                            i32.const 624
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 0
                            i32.load
                            i32.store
                            local.get 1
                            local.get 1
                            i64.load offset=1244 align=4
                            i64.store offset=624
                            local.get 1
                            i32.const 256
                            i32.add
                            local.get 1
                            i32.const 432
                            i32.add
                            call $_ZN56_$LT$u8$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h374e211fca1804deE
                            i32.const 1
                            local.set 0
                            local.get 1
                            i32.load8_u offset=256
                            i32.const 1
                            i32.and
                            br_if 6 (;@6;)
                            local.get 1
                            i32.load8_u offset=257
                            local.set 4
                            local.get 1
                            i32.const 480
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 1
                            i32.const 520
                            i32.add
                            i32.const 8
                            i32.add
                            i32.load
                            i32.store
                            local.get 1
                            i32.const 392
                            i32.add
                            i32.const 8
                            i32.add
                            local.get 1
                            i32.const 624
                            i32.add
                            i32.const 8
                            i32.add
                            i32.load
                            i32.store
                            local.get 1
                            local.get 1
                            i64.load offset=520
                            i64.store offset=480
                            local.get 1
                            local.get 1
                            i64.load offset=624
                            i64.store offset=392
                            i32.const 0
                            local.set 0
                            br 7 (;@5;)
                          end
                          local.get 1
                          i32.const 16384
                          i32.store offset=468
                          local.get 1
                          i32.const 68528
                          i32.store offset=464
                          local.get 1
                          i32.const 464
                          i32.add
                          call $_ZN7ink_env6engine8on_chain3ext5input17h026d86c7dec708cfE
                          local.get 1
                          local.get 1
                          i64.load offset=464
                          i64.store offset=432
                          i32.const 1
                          local.set 0
                          i32.const 0
                          local.set 5
                          local.get 1
                          i32.const 432
                          i32.add
                          call $_ZN76_$LT$$u5b$T$u3b$$u20$N$u5d$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h6fb3ac2ce221d964E
                          local.tee 3
                          i32.wrap_i64
                          local.tee 4
                          i32.const 1
                          i32.and
                          br_if 3 (;@8;)
                          local.get 3
                          i64.const 1099511627775
                          i64.and
                          local.tee 3
                          i64.const 32
                          i64.shr_u
                          i32.wrap_i64
                          local.set 6
                          local.get 3
                          i64.const 24
                          i64.shr_u
                          i32.wrap_i64
                          local.set 7
                          local.get 3
                          i64.const 16
                          i64.shr_u
                          i32.wrap_i64
                          local.set 8
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              block  ;; label = @22
                                                block  ;; label = @23
                                                  block  ;; label = @24
                                                    block  ;; label = @25
                                                      local.get 4
                                                      i32.const 8
                                                      i32.shr_u
                                                      i32.const 255
                                                      i32.and
                                                      local.tee 4
                                                      i32.const -22
                                                      i32.add
                                                      br_table 1 (;@24;) 18 (;@7;) 18 (;@7;) 11 (;@14;) 0 (;@25;)
                                                    end
                                                    local.get 4
                                                    i32.const 52
                                                    i32.eq
                                                    br_if 7 (;@17;)
                                                    local.get 4
                                                    i32.const 61
                                                    i32.eq
                                                    br_if 9 (;@15;)
                                                    local.get 4
                                                    i32.const 77
                                                    i32.eq
                                                    br_if 6 (;@18;)
                                                    local.get 4
                                                    i32.const 84
                                                    i32.eq
                                                    br_if 2 (;@22;)
                                                    local.get 4
                                                    i32.const 101
                                                    i32.eq
                                                    br_if 1 (;@23;)
                                                    local.get 4
                                                    i32.const 114
                                                    i32.eq
                                                    br_if 8 (;@16;)
                                                    local.get 4
                                                    i32.const 150
                                                    i32.eq
                                                    br_if 5 (;@19;)
                                                    local.get 4
                                                    i32.const 158
                                                    i32.eq
                                                    br_if 11 (;@13;)
                                                    local.get 4
                                                    i32.const 178
                                                    i32.eq
                                                    br_if 4 (;@20;)
                                                    local.get 4
                                                    i32.const 219
                                                    i32.eq
                                                    br_if 3 (;@21;)
                                                    local.get 4
                                                    i32.const 254
                                                    i32.ne
                                                    br_if 16 (;@8;)
                                                    local.get 8
                                                    i32.const 255
                                                    i32.and
                                                    i32.const 203
                                                    i32.ne
                                                    br_if 16 (;@8;)
                                                    local.get 7
                                                    i32.const 255
                                                    i32.and
                                                    i32.const 87
                                                    i32.ne
                                                    br_if 16 (;@8;)
                                                    local.get 6
                                                    i32.const 213
                                                    i32.ne
                                                    br_if 16 (;@8;)
                                                    local.get 1
                                                    i32.const 1240
                                                    i32.add
                                                    local.get 1
                                                    i32.const 432
                                                    i32.add
                                                    call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                                    i32.const 1
                                                    local.set 0
                                                    local.get 1
                                                    i32.load8_u offset=1240
                                                    i32.const 1
                                                    i32.eq
                                                    br_if 16 (;@8;)
                                                    local.get 1
                                                    i32.const 624
                                                    i32.add
                                                    i32.const 24
                                                    i32.add
                                                    local.tee 0
                                                    local.get 1
                                                    i32.const 1265
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 1
                                                    i32.const 624
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.tee 4
                                                    local.get 1
                                                    i32.const 1257
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 1
                                                    i32.const 624
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.tee 9
                                                    local.get 1
                                                    i32.const 1249
                                                    i32.add
                                                    i64.load align=1
                                                    i64.store
                                                    local.get 1
                                                    local.get 1
                                                    i64.load offset=1241 align=1
                                                    i64.store offset=624
                                                    local.get 1
                                                    i32.const 176
                                                    i32.add
                                                    local.get 1
                                                    i32.const 432
                                                    i32.add
                                                    call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
                                                    local.get 1
                                                    i64.load offset=176
                                                    i32.wrap_i64
                                                    br_if 14 (;@10;)
                                                    local.get 1
                                                    i32.const 176
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    i64.load
                                                    local.set 3
                                                    local.get 1
                                                    i64.load offset=184
                                                    local.set 2
                                                    local.get 1
                                                    i32.const 520
                                                    i32.add
                                                    i32.const 24
                                                    i32.add
                                                    local.get 0
                                                    i64.load
                                                    i64.store
                                                    local.get 1
                                                    i32.const 520
                                                    i32.add
                                                    i32.const 16
                                                    i32.add
                                                    local.get 4
                                                    i64.load
                                                    i64.store
                                                    local.get 1
                                                    i32.const 520
                                                    i32.add
                                                    i32.const 8
                                                    i32.add
                                                    local.get 9
                                                    i64.load
                                                    i64.store
                                                    local.get 1
                                                    local.get 1
                                                    i64.load offset=624
                                                    i64.store offset=520
                                                    local.get 1
                                                    local.get 2
                                                    i64.store offset=592
                                                    local.get 1
                                                    local.get 3
                                                    i64.store offset=600
                                                    i32.const 5
                                                    local.set 4
                                                    br 12 (;@12;)
                                                  end
                                                  i32.const 0
                                                  local.set 4
                                                  local.get 8
                                                  i32.const 255
                                                  i32.and
                                                  i32.const 45
                                                  i32.ne
                                                  br_if 14 (;@9;)
                                                  local.get 7
                                                  i32.const 255
                                                  i32.and
                                                  i32.const 248
                                                  i32.ne
                                                  br_if 14 (;@9;)
                                                  i32.const 0
                                                  local.set 5
                                                  local.get 6
                                                  i32.const 194
                                                  i32.eq
                                                  br_if 11 (;@12;)
                                                  br 16 (;@7;)
                                                end
                                                local.get 8
                                                i32.const 255
                                                i32.and
                                                i32.const 104
                                                i32.ne
                                                br_if 14 (;@8;)
                                                local.get 7
                                                i32.const 255
                                                i32.and
                                                i32.const 56
                                                i32.ne
                                                br_if 14 (;@8;)
                                                local.get 6
                                                i32.const 47
                                                i32.ne
                                                br_if 14 (;@8;)
                                                local.get 1
                                                i32.const 1240
                                                i32.add
                                                local.get 1
                                                i32.const 432
                                                i32.add
                                                call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                                i32.const 1
                                                local.set 4
                                                local.get 1
                                                i32.load8_u offset=1240
                                                i32.const 1
                                                i32.eq
                                                br_if 12 (;@10;)
                                                local.get 1
                                                i32.const 544
                                                i32.add
                                                local.get 1
                                                i32.const 1265
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 1
                                                i32.const 536
                                                i32.add
                                                local.get 1
                                                i32.const 1257
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 1
                                                i32.const 528
                                                i32.add
                                                local.get 1
                                                i32.const 1249
                                                i32.add
                                                i64.load align=1
                                                i64.store
                                                local.get 1
                                                local.get 1
                                                i64.load offset=1241 align=1
                                                i64.store offset=520
                                                br 10 (;@12;)
                                              end
                                              local.get 8
                                              i32.const 255
                                              i32.and
                                              i32.const 179
                                              i32.ne
                                              br_if 13 (;@8;)
                                              local.get 7
                                              i32.const 255
                                              i32.and
                                              i32.const 199
                                              i32.ne
                                              br_if 13 (;@8;)
                                              local.get 6
                                              i32.const 110
                                              i32.ne
                                              br_if 13 (;@8;)
                                              local.get 1
                                              i32.const 1240
                                              i32.add
                                              local.get 1
                                              i32.const 432
                                              i32.add
                                              call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                              i32.const 1
                                              local.set 0
                                              local.get 1
                                              i32.load8_u offset=1240
                                              i32.const 1
                                              i32.eq
                                              br_if 13 (;@8;)
                                              local.get 1
                                              i32.const 624
                                              i32.add
                                              i32.const 24
                                              i32.add
                                              local.get 1
                                              i32.const 1265
                                              i32.add
                                              i64.load align=1
                                              i64.store
                                              local.get 1
                                              i32.const 624
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              local.get 1
                                              i32.const 1257
                                              i32.add
                                              i64.load align=1
                                              i64.store
                                              local.get 1
                                              i32.const 624
                                              i32.add
                                              i32.const 8
                                              i32.add
                                              local.get 1
                                              i32.const 1249
                                              i32.add
                                              i64.load align=1
                                              i64.store
                                              local.get 1
                                              local.get 1
                                              i64.load offset=1241 align=1
                                              i64.store offset=624
                                              local.get 1
                                              i32.const 1240
                                              i32.add
                                              local.get 1
                                              i32.const 432
                                              i32.add
                                              call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                              local.get 1
                                              i32.load8_u offset=1240
                                              i32.const 1
                                              i32.eq
                                              br_if 11 (;@10;)
                                              local.get 1
                                              i32.const 390
                                              i32.add
                                              local.get 1
                                              i32.load8_u offset=1243
                                              i32.store8
                                              local.get 1
                                              i32.const 392
                                              i32.add
                                              i32.const 8
                                              i32.add
                                              local.get 1
                                              i32.const 1240
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              i64.load
                                              i64.store
                                              local.get 1
                                              i32.const 392
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              local.get 1
                                              i32.const 1240
                                              i32.add
                                              i32.const 24
                                              i32.add
                                              i64.load
                                              i64.store
                                              local.get 1
                                              local.get 1
                                              i32.load16_u offset=1241 align=1
                                              i32.store16 offset=388
                                              local.get 1
                                              local.get 1
                                              i32.const 1240
                                              i32.add
                                              i32.const 8
                                              i32.add
                                              i64.load
                                              i64.store offset=392
                                              local.get 1
                                              i32.const 1272
                                              i32.add
                                              i32.load8_u
                                              local.set 10
                                              local.get 1
                                              i32.load offset=1244
                                              local.set 9
                                              local.get 1
                                              i32.const 104
                                              i32.add
                                              local.get 1
                                              i32.const 432
                                              i32.add
                                              call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
                                              local.get 1
                                              i32.load offset=104
                                              br_if 11 (;@10;)
                                              local.get 1
                                              i32.const 104
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              i64.load
                                              local.set 3
                                              local.get 1
                                              i64.load offset=112
                                              local.set 2
                                              local.get 1
                                              i32.const 1240
                                              i32.add
                                              local.get 1
                                              i32.const 432
                                              i32.add
                                              call $_ZN78_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17hf1fcdcd6d2b09f13E
                                              local.get 1
                                              i32.load offset=1240
                                              local.tee 11
                                              i32.eqz
                                              br_if 11 (;@10;)
                                              local.get 1
                                              i32.const 520
                                              i32.add
                                              i32.const 8
                                              i32.add
                                              local.get 1
                                              i32.const 624
                                              i32.add
                                              i32.const 8
                                              i32.add
                                              i64.load
                                              i64.store
                                              local.get 1
                                              i32.const 520
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              local.get 1
                                              i32.const 624
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              i64.load
                                              i64.store
                                              local.get 1
                                              i32.const 520
                                              i32.add
                                              i32.const 24
                                              i32.add
                                              local.get 1
                                              i32.const 624
                                              i32.add
                                              i32.const 24
                                              i32.add
                                              i64.load
                                              i64.store
                                              i32.const 2
                                              local.set 4
                                              local.get 1
                                              i32.const 384
                                              i32.add
                                              i32.const 2
                                              i32.add
                                              local.get 1
                                              i32.const 388
                                              i32.add
                                              i32.const 2
                                              i32.add
                                              i32.load8_u
                                              i32.store8
                                              local.get 1
                                              i32.const 592
                                              i32.add
                                              i32.const 8
                                              i32.add
                                              local.get 1
                                              i32.const 392
                                              i32.add
                                              i32.const 8
                                              i32.add
                                              i64.load
                                              i64.store
                                              local.get 1
                                              i32.const 592
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              local.get 1
                                              i32.const 392
                                              i32.add
                                              i32.const 16
                                              i32.add
                                              i64.load
                                              i64.store
                                              local.get 1
                                              local.get 1
                                              i64.load offset=624
                                              i64.store offset=520
                                              local.get 1
                                              local.get 1
                                              i32.load16_u offset=388
                                              i32.store16 offset=384
                                              local.get 1
                                              local.get 1
                                              i64.load offset=392
                                              i64.store offset=592
                                              local.get 1
                                              i64.load offset=1244 align=4
                                              local.set 12
                                              br 9 (;@12;)
                                            end
                                            local.get 8
                                            i32.const 255
                                            i32.and
                                            i32.const 32
                                            i32.ne
                                            br_if 12 (;@8;)
                                            local.get 7
                                            i32.const 255
                                            i32.and
                                            i32.const 249
                                            i32.ne
                                            br_if 12 (;@8;)
                                            local.get 6
                                            i32.const 245
                                            i32.ne
                                            br_if 12 (;@8;)
                                            local.get 1
                                            i32.const 1240
                                            i32.add
                                            local.get 1
                                            i32.const 432
                                            i32.add
                                            call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                            i32.const 1
                                            local.set 0
                                            local.get 1
                                            i32.load8_u offset=1240
                                            i32.const 1
                                            i32.eq
                                            br_if 12 (;@8;)
                                            local.get 1
                                            i32.const 648
                                            i32.add
                                            local.get 1
                                            i32.const 1265
                                            i32.add
                                            i64.load align=1
                                            i64.store
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            i32.const 16
                                            i32.add
                                            local.get 1
                                            i32.const 1257
                                            i32.add
                                            i64.load align=1
                                            i64.store
                                            local.get 1
                                            i32.const 632
                                            i32.add
                                            local.get 1
                                            i32.const 1249
                                            i32.add
                                            i64.load align=1
                                            i64.store
                                            local.get 1
                                            local.get 1
                                            i64.load offset=1241 align=1
                                            i64.store offset=624
                                            local.get 1
                                            i32.const 128
                                            i32.add
                                            local.get 1
                                            i32.const 432
                                            i32.add
                                            call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
                                            local.get 1
                                            i32.load offset=128
                                            br_if 10 (;@10;)
                                            local.get 1
                                            i32.const 128
                                            i32.add
                                            i32.const 16
                                            i32.add
                                            i64.load
                                            local.set 3
                                            local.get 1
                                            i64.load offset=136
                                            local.set 2
                                            local.get 1
                                            i32.const 1240
                                            i32.add
                                            local.get 1
                                            i32.const 432
                                            i32.add
                                            call $_ZN78_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17hf1fcdcd6d2b09f13E
                                            local.get 1
                                            i32.load offset=1240
                                            local.tee 9
                                            i32.eqz
                                            br_if 10 (;@10;)
                                            local.get 1
                                            i32.const 520
                                            i32.add
                                            i32.const 8
                                            i32.add
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            i32.const 8
                                            i32.add
                                            i64.load
                                            i64.store
                                            local.get 1
                                            i32.const 520
                                            i32.add
                                            i32.const 16
                                            i32.add
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            i32.const 16
                                            i32.add
                                            i64.load
                                            i64.store
                                            local.get 1
                                            i32.const 520
                                            i32.add
                                            i32.const 24
                                            i32.add
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            i32.const 24
                                            i32.add
                                            i64.load
                                            i64.store
                                            local.get 1
                                            i32.const 592
                                            i32.add
                                            i32.const 16
                                            i32.add
                                            local.get 3
                                            i64.store
                                            local.get 1
                                            local.get 1
                                            i64.load offset=624
                                            i64.store offset=520
                                            local.get 1
                                            local.get 2
                                            i64.store offset=600
                                            local.get 1
                                            local.get 1
                                            i64.load offset=1244 align=4
                                            i64.store offset=592
                                            i32.const 3
                                            local.set 4
                                            br 8 (;@12;)
                                          end
                                          local.get 8
                                          i32.const 255
                                          i32.and
                                          i32.const 15
                                          i32.ne
                                          br_if 11 (;@8;)
                                          local.get 7
                                          i32.const 255
                                          i32.and
                                          i32.const 27
                                          i32.ne
                                          br_if 11 (;@8;)
                                          local.get 6
                                          i32.const 189
                                          i32.ne
                                          br_if 11 (;@8;)
                                          local.get 1
                                          i32.const 1240
                                          i32.add
                                          local.get 1
                                          i32.const 432
                                          i32.add
                                          call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                          i32.const 1
                                          local.set 0
                                          local.get 1
                                          i32.load8_u offset=1240
                                          i32.const 1
                                          i32.eq
                                          br_if 11 (;@8;)
                                          local.get 1
                                          i32.const 624
                                          i32.add
                                          i32.const 24
                                          i32.add
                                          local.tee 0
                                          local.get 1
                                          i32.const 1265
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 1
                                          i32.const 624
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          local.tee 4
                                          local.get 1
                                          i32.const 1257
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 1
                                          i32.const 624
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          local.tee 9
                                          local.get 1
                                          i32.const 1249
                                          i32.add
                                          i64.load align=1
                                          i64.store
                                          local.get 1
                                          local.get 1
                                          i64.load offset=1241 align=1
                                          i64.store offset=624
                                          local.get 1
                                          i32.const 152
                                          i32.add
                                          local.get 1
                                          i32.const 432
                                          i32.add
                                          call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
                                          local.get 1
                                          i64.load offset=152
                                          i32.wrap_i64
                                          br_if 9 (;@10;)
                                          local.get 1
                                          i32.const 152
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          i64.load
                                          local.set 3
                                          local.get 1
                                          i64.load offset=160
                                          local.set 2
                                          local.get 1
                                          i32.const 520
                                          i32.add
                                          i32.const 24
                                          i32.add
                                          local.get 0
                                          i64.load
                                          i64.store
                                          local.get 1
                                          i32.const 520
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          local.get 4
                                          i64.load
                                          i64.store
                                          local.get 1
                                          i32.const 520
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          local.get 9
                                          i64.load
                                          i64.store
                                          local.get 1
                                          local.get 1
                                          i64.load offset=624
                                          i64.store offset=520
                                          local.get 1
                                          local.get 2
                                          i64.store offset=592
                                          local.get 1
                                          local.get 3
                                          i64.store offset=600
                                          i32.const 4
                                          local.set 4
                                          br 7 (;@12;)
                                        end
                                        local.get 8
                                        i32.const 255
                                        i32.and
                                        i32.const 214
                                        i32.ne
                                        br_if 10 (;@8;)
                                        local.get 7
                                        i32.const 255
                                        i32.and
                                        i32.const 181
                                        i32.ne
                                        br_if 10 (;@8;)
                                        local.get 6
                                        i32.const 122
                                        i32.ne
                                        br_if 10 (;@8;)
                                        local.get 1
                                        i32.const 1240
                                        i32.add
                                        local.get 1
                                        i32.const 432
                                        i32.add
                                        call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                        i32.const 1
                                        local.set 0
                                        local.get 1
                                        i32.load8_u offset=1240
                                        i32.const 1
                                        i32.eq
                                        br_if 10 (;@8;)
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.tee 0
                                        local.get 1
                                        i32.const 1265
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.tee 4
                                        local.get 1
                                        i32.const 1257
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.tee 9
                                        local.get 1
                                        i32.const 1249
                                        i32.add
                                        i64.load align=1
                                        i64.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=1241 align=1
                                        i64.store offset=624
                                        local.get 1
                                        i32.const 200
                                        i32.add
                                        local.get 1
                                        i32.const 432
                                        i32.add
                                        call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
                                        local.get 1
                                        i64.load offset=200
                                        i32.wrap_i64
                                        br_if 8 (;@10;)
                                        local.get 1
                                        i32.const 200
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i64.load
                                        local.set 3
                                        local.get 1
                                        i64.load offset=208
                                        local.set 2
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.get 0
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.get 4
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.get 9
                                        i64.load
                                        i64.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=624
                                        i64.store offset=520
                                        local.get 1
                                        local.get 2
                                        i64.store offset=592
                                        local.get 1
                                        local.get 3
                                        i64.store offset=600
                                        i32.const 6
                                        local.set 4
                                        br 6 (;@12;)
                                      end
                                      local.get 8
                                      i32.const 255
                                      i32.and
                                      i32.const 71
                                      i32.ne
                                      br_if 9 (;@8;)
                                      local.get 7
                                      i32.const 255
                                      i32.and
                                      i32.const 217
                                      i32.ne
                                      br_if 9 (;@8;)
                                      local.get 6
                                      i32.const 33
                                      i32.ne
                                      br_if 9 (;@8;)
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      local.get 1
                                      i32.const 432
                                      i32.add
                                      call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                      i32.const 1
                                      local.set 0
                                      local.get 1
                                      i32.load8_u offset=1240
                                      i32.const 1
                                      i32.eq
                                      br_if 9 (;@8;)
                                      local.get 1
                                      i32.const 624
                                      i32.add
                                      i32.const 24
                                      i32.add
                                      local.tee 0
                                      local.get 1
                                      i32.const 1265
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 1
                                      i32.const 624
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.tee 4
                                      local.get 1
                                      i32.const 1257
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 1
                                      i32.const 624
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.tee 9
                                      local.get 1
                                      i32.const 1249
                                      i32.add
                                      i64.load align=1
                                      i64.store
                                      local.get 1
                                      local.get 1
                                      i64.load offset=1241 align=1
                                      i64.store offset=624
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      local.get 1
                                      i32.const 432
                                      i32.add
                                      call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                                      local.get 1
                                      i32.load8_u offset=1240
                                      i32.const 1
                                      i32.eq
                                      br_if 7 (;@10;)
                                      local.get 1
                                      i32.const 386
                                      i32.add
                                      local.get 1
                                      i32.load8_u offset=1243
                                      i32.store8
                                      local.get 1
                                      i32.const 592
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 592
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      i32.const 24
                                      i32.add
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 520
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 9
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 520
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 520
                                      i32.add
                                      i32.const 24
                                      i32.add
                                      local.get 0
                                      i64.load
                                      i64.store
                                      local.get 1
                                      local.get 1
                                      i32.load16_u offset=1241 align=1
                                      i32.store16 offset=384
                                      local.get 1
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      i64.load
                                      i64.store offset=592
                                      local.get 1
                                      local.get 1
                                      i64.load offset=624
                                      i64.store offset=520
                                      local.get 1
                                      i32.const 1272
                                      i32.add
                                      i32.load8_u
                                      local.set 10
                                      local.get 1
                                      i32.load offset=1244
                                      local.set 9
                                      i32.const 7
                                      local.set 4
                                      br 5 (;@12;)
                                    end
                                    local.get 8
                                    i32.const 255
                                    i32.and
                                    i32.const 32
                                    i32.ne
                                    br_if 8 (;@8;)
                                    local.get 7
                                    i32.const 255
                                    i32.and
                                    i32.const 91
                                    i32.ne
                                    br_if 8 (;@8;)
                                    i32.const 8
                                    local.set 4
                                    local.get 6
                                    i32.const 229
                                    i32.eq
                                    br_if 4 (;@12;)
                                    br 9 (;@7;)
                                  end
                                  local.get 8
                                  i32.const 255
                                  i32.and
                                  i32.const 113
                                  i32.ne
                                  br_if 7 (;@8;)
                                  local.get 7
                                  i32.const 255
                                  i32.and
                                  i32.const 183
                                  i32.ne
                                  br_if 7 (;@8;)
                                  i32.const 9
                                  local.set 4
                                  local.get 6
                                  i32.const 130
                                  i32.eq
                                  br_if 3 (;@12;)
                                  br 8 (;@7;)
                                end
                                local.get 8
                                i32.const 255
                                i32.and
                                i32.const 38
                                i32.ne
                                br_if 6 (;@8;)
                                local.get 7
                                i32.const 255
                                i32.and
                                i32.const 27
                                i32.ne
                                br_if 6 (;@8;)
                                i32.const 10
                                local.set 4
                                local.get 6
                                i32.const 212
                                i32.eq
                                br_if 2 (;@12;)
                                br 7 (;@7;)
                              end
                              local.get 8
                              i32.const 255
                              i32.and
                              i32.const 57
                              i32.ne
                              br_if 5 (;@8;)
                              local.get 7
                              i32.const 255
                              i32.and
                              i32.const 137
                              i32.ne
                              br_if 5 (;@8;)
                              local.get 6
                              i32.const 98
                              i32.ne
                              br_if 5 (;@8;)
                              local.get 1
                              i32.const 1240
                              i32.add
                              local.get 1
                              i32.const 432
                              i32.add
                              call $_ZN7ink_env5types1_89_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_env..types..AccountId$GT$6decode17hafbc4d48c77adab6E
                              i32.const 1
                              local.set 0
                              local.get 1
                              i32.load8_u offset=1240
                              i32.const 1
                              i32.eq
                              br_if 5 (;@8;)
                              local.get 1
                              i32.const 544
                              i32.add
                              local.get 1
                              i32.const 1265
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 1
                              i32.const 536
                              i32.add
                              local.get 1
                              i32.const 1257
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 1
                              i32.const 528
                              i32.add
                              local.get 1
                              i32.const 1249
                              i32.add
                              i64.load align=1
                              i64.store
                              local.get 1
                              local.get 1
                              i64.load offset=1241 align=1
                              i64.store offset=520
                              i32.const 11
                              local.set 4
                              br 1 (;@12;)
                            end
                            local.get 8
                            i32.const 255
                            i32.and
                            i32.const 94
                            i32.ne
                            br_if 4 (;@8;)
                            local.get 7
                            i32.const 255
                            i32.and
                            i32.const 54
                            i32.ne
                            br_if 4 (;@8;)
                            i32.const 12
                            local.set 4
                            local.get 6
                            i32.const 63
                            i32.ne
                            br_if 5 (;@7;)
                          end
                          local.get 1
                          i32.const 480
                          i32.add
                          i32.const 24
                          i32.add
                          local.get 1
                          i32.const 520
                          i32.add
                          i32.const 24
                          i32.add
                          i64.load
                          i64.store
                          local.get 1
                          i32.const 480
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 1
                          i32.const 520
                          i32.add
                          i32.const 16
                          i32.add
                          i64.load
                          i64.store
                          local.get 1
                          i32.const 480
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 1
                          i32.const 520
                          i32.add
                          i32.const 8
                          i32.add
                          i64.load
                          i64.store
                          local.get 1
                          i32.const 380
                          i32.add
                          i32.const 2
                          i32.add
                          local.get 1
                          i32.const 384
                          i32.add
                          i32.const 2
                          i32.add
                          i32.load8_u
                          i32.store8
                          local.get 1
                          i32.const 560
                          i32.add
                          i32.const 8
                          i32.add
                          local.get 1
                          i32.const 592
                          i32.add
                          i32.const 8
                          i32.add
                          i64.load
                          i64.store
                          local.get 1
                          i32.const 560
                          i32.add
                          i32.const 16
                          i32.add
                          local.get 1
                          i32.const 592
                          i32.add
                          i32.const 16
                          i32.add
                          i64.load
                          i64.store
                          local.get 1
                          local.get 1
                          i64.load offset=520
                          i64.store offset=480
                          local.get 1
                          local.get 1
                          i32.load16_u offset=384
                          i32.store16 offset=380
                          local.get 1
                          local.get 1
                          i64.load offset=592
                          i64.store offset=560
                          i32.const 0
                          local.set 0
                          local.get 4
                          local.set 5
                          br 4 (;@7;)
                        end
                        i32.const 1
                        local.set 0
                        br 5 (;@5;)
                      end
                      i32.const 1
                      local.set 0
                      br 2 (;@7;)
                    end
                    i32.const 0
                    local.set 5
                    br 1 (;@7;)
                  end
                end
                local.get 0
                br_if 2 (;@4;)
                local.get 1
                i32.const 344
                i32.add
                i32.const 24
                i32.add
                local.tee 0
                local.get 1
                i32.const 480
                i32.add
                i32.const 24
                i32.add
                local.tee 13
                i64.load
                i64.store
                local.get 1
                i32.const 344
                i32.add
                i32.const 16
                i32.add
                local.tee 4
                local.get 1
                i32.const 480
                i32.add
                i32.const 16
                i32.add
                local.tee 14
                i64.load
                i64.store
                local.get 1
                i32.const 344
                i32.add
                i32.const 8
                i32.add
                local.tee 8
                local.get 1
                i32.const 480
                i32.add
                i32.const 8
                i32.add
                local.tee 15
                i64.load
                i64.store
                local.get 1
                i32.const 340
                i32.add
                i32.const 2
                i32.add
                local.tee 16
                local.get 1
                i32.const 380
                i32.add
                i32.const 2
                i32.add
                i32.load8_u
                i32.store8
                local.get 1
                i32.const 312
                i32.add
                i32.const 8
                i32.add
                local.tee 7
                local.get 1
                i32.const 560
                i32.add
                i32.const 8
                i32.add
                local.tee 17
                i64.load
                i64.store
                local.get 1
                i32.const 312
                i32.add
                i32.const 16
                i32.add
                local.tee 6
                local.get 1
                i32.const 560
                i32.add
                i32.const 16
                i32.add
                local.tee 18
                i64.load
                i64.store
                local.get 1
                local.get 1
                i64.load offset=480
                i64.store offset=344
                local.get 1
                local.get 1
                i32.load16_u offset=380
                i32.store16 offset=340
                local.get 1
                local.get 1
                i64.load offset=560
                i64.store offset=312
                local.get 1
                i32.const 288
                i32.add
                i32.const 16
                i32.add
                local.tee 19
                local.get 6
                i64.load
                i64.store
                local.get 1
                i32.const 288
                i32.add
                i32.const 8
                i32.add
                local.tee 20
                local.get 7
                i64.load
                i64.store
                local.get 1
                local.get 1
                i64.load offset=312
                i64.store offset=288
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    block  ;; label = @17
                                      block  ;; label = @18
                                        block  ;; label = @19
                                          block  ;; label = @20
                                            block  ;; label = @21
                                              local.get 5
                                              br_table 0 (;@21;) 1 (;@20;) 2 (;@19;) 12 (;@9;) 3 (;@18;) 4 (;@17;) 5 (;@16;) 6 (;@15;) 7 (;@14;) 8 (;@13;) 9 (;@12;) 10 (;@11;) 11 (;@10;) 0 (;@21;)
                                            end
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            i32.const 24
                                            i32.add
                                            i64.const 0
                                            i64.store
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            i32.const 16
                                            i32.add
                                            i64.const 0
                                            i64.store
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            i32.const 8
                                            i32.add
                                            i64.const 0
                                            i64.store
                                            local.get 1
                                            i64.const 0
                                            i64.store offset=624
                                            local.get 1
                                            i32.const 1240
                                            i32.add
                                            local.get 1
                                            i32.const 624
                                            i32.add
                                            call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                                            local.get 1
                                            i32.const 8
                                            i32.add
                                            local.get 1
                                            i32.const 1240
                                            i32.add
                                            call $_ZN5psp226traits5PSP2212total_supply17h3afc31b33677089bE
                                            local.get 1
                                            local.get 1
                                            i32.const 8
                                            i32.add
                                            i32.const 8
                                            i32.add
                                            i64.load
                                            i64.store offset=528
                                            local.get 1
                                            local.get 1
                                            i64.load offset=8
                                            i64.store offset=520
                                            local.get 1
                                            i32.const 520
                                            i32.add
                                            call $_ZN7ink_env3api12return_value17h30fd262fc41c2a34E
                                            unreachable
                                          end
                                          local.get 1
                                          i32.const 520
                                          i32.add
                                          i32.const 24
                                          i32.add
                                          i64.const 0
                                          i64.store
                                          local.get 1
                                          i32.const 520
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          i64.const 0
                                          i64.store
                                          local.get 1
                                          i32.const 520
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          i64.const 0
                                          i64.store
                                          local.get 1
                                          i64.const 0
                                          i64.store offset=520
                                          local.get 1
                                          i32.const 1240
                                          i32.add
                                          local.get 1
                                          i32.const 520
                                          i32.add
                                          call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                                          local.get 1
                                          i32.const 624
                                          i32.add
                                          i32.const 24
                                          i32.add
                                          local.get 0
                                          i64.load
                                          i64.store
                                          local.get 1
                                          i32.const 624
                                          i32.add
                                          i32.const 16
                                          i32.add
                                          local.get 4
                                          i64.load
                                          i64.store
                                          local.get 1
                                          i32.const 624
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          local.get 8
                                          i64.load
                                          i64.store
                                          local.get 1
                                          local.get 1
                                          i64.load offset=344
                                          i64.store offset=624
                                          local.get 1
                                          i32.const 24
                                          i32.add
                                          local.get 1
                                          i32.const 1240
                                          i32.add
                                          local.get 1
                                          i32.const 624
                                          i32.add
                                          call $_ZN5psp226traits5PSP2210balance_of17h8e8fb26b27704ba9E
                                          local.get 1
                                          local.get 1
                                          i32.const 24
                                          i32.add
                                          i32.const 8
                                          i32.add
                                          i64.load
                                          i64.store offset=488
                                          local.get 1
                                          local.get 1
                                          i64.load offset=24
                                          i64.store offset=480
                                          local.get 1
                                          i32.const 480
                                          i32.add
                                          call $_ZN7ink_env3api12return_value17h30fd262fc41c2a34E
                                          unreachable
                                        end
                                        local.get 1
                                        i32.const 392
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        i64.const 0
                                        i64.store
                                        local.get 1
                                        i32.const 392
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i64.const 0
                                        i64.store
                                        local.get 1
                                        i32.const 392
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        i64.const 0
                                        i64.store
                                        local.get 1
                                        i64.const 0
                                        i64.store offset=392
                                        local.get 1
                                        i32.const 1240
                                        i32.add
                                        local.get 1
                                        i32.const 392
                                        i32.add
                                        call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                                        local.get 1
                                        i32.const 480
                                        i32.add
                                        call $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.get 0
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.get 4
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.get 8
                                        i64.load
                                        i64.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=344
                                        i64.store offset=520
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.get 13
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.get 14
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.get 15
                                        i64.load
                                        i64.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=480
                                        i64.store offset=624
                                        local.get 1
                                        i32.const 40
                                        i32.add
                                        local.get 1
                                        i32.const 1240
                                        i32.add
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        call $_ZN5psp226traits5PSP229allowance17h133c5b43bbd89b5bE
                                        local.get 1
                                        i64.load offset=40
                                        local.tee 21
                                        local.get 2
                                        i64.ge_u
                                        local.get 1
                                        i32.const 40
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        i64.load
                                        local.tee 22
                                        local.get 3
                                        i64.ge_u
                                        local.get 22
                                        local.get 3
                                        i64.eq
                                        select
                                        i32.eqz
                                        br_if 16 (;@2;)
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.tee 0
                                        local.get 1
                                        i32.const 344
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.tee 4
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.tee 5
                                        local.get 1
                                        i32.const 344
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.tee 8
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.tee 7
                                        local.get 1
                                        i32.const 344
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.tee 6
                                        i64.load
                                        i64.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=344
                                        i64.store offset=520
                                        local.get 1
                                        i32.const 639
                                        i32.add
                                        local.get 1
                                        i32.const 312
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        i64.load
                                        i64.store align=1
                                        local.get 1
                                        i32.const 647
                                        i32.add
                                        local.get 1
                                        i32.const 312
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i64.load
                                        i64.store align=1
                                        local.get 1
                                        local.get 1
                                        i32.const 342
                                        i32.add
                                        i32.load8_u
                                        i32.store8 offset=626
                                        local.get 1
                                        local.get 1
                                        i32.load16_u offset=340
                                        i32.store16 offset=624
                                        local.get 1
                                        local.get 9
                                        i32.store offset=627 align=1
                                        local.get 1
                                        local.get 1
                                        i64.load offset=312
                                        i64.store offset=631 align=1
                                        local.get 1
                                        local.get 10
                                        i32.store8 offset=655
                                        local.get 1
                                        local.get 12
                                        i64.store offset=596 align=4
                                        local.get 1
                                        local.get 11
                                        i32.store offset=592
                                        local.get 1
                                        i32.const 1240
                                        i32.add
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        local.get 2
                                        local.get 3
                                        local.get 1
                                        i32.const 592
                                        i32.add
                                        call $_ZN5psp226traits5PSP2217_transfer_from_to17h81f2dfe081d11fc0E
                                        local.get 0
                                        local.get 4
                                        i64.load
                                        i64.store
                                        local.get 5
                                        local.get 8
                                        i64.load
                                        i64.store
                                        local.get 7
                                        local.get 6
                                        i64.load
                                        i64.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=344
                                        i64.store offset=520
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        local.get 1
                                        i32.const 480
                                        i32.add
                                        i32.const 24
                                        i32.add
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        local.get 1
                                        i32.const 480
                                        i32.add
                                        i32.const 16
                                        i32.add
                                        i64.load
                                        i64.store
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        local.get 1
                                        i32.const 480
                                        i32.add
                                        i32.const 8
                                        i32.add
                                        i64.load
                                        i64.store
                                        local.get 1
                                        local.get 1
                                        i64.load offset=480
                                        i64.store offset=624
                                        local.get 1
                                        i32.const 1240
                                        i32.add
                                        local.get 1
                                        i32.const 520
                                        i32.add
                                        local.get 1
                                        i32.const 624
                                        i32.add
                                        local.get 21
                                        local.get 2
                                        i64.sub
                                        local.get 22
                                        local.get 3
                                        i64.sub
                                        local.get 21
                                        local.get 2
                                        i64.lt_u
                                        i64.extend_i32_u
                                        i64.sub
                                        call $_ZN5psp226traits5PSP2216_approve_from_to17hc115ddc6234276ffE
                                        local.get 1
                                        i32.const 1240
                                        i32.add
                                        local.get 1
                                        i32.const 392
                                        i32.add
                                        call $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E
                                        br 10 (;@8;)
                                      end
                                      local.get 20
                                      i64.load
                                      local.set 3
                                      local.get 1
                                      i64.load offset=288
                                      local.set 2
                                      local.get 1
                                      i32.const 392
                                      i32.add
                                      i32.const 24
                                      i32.add
                                      i64.const 0
                                      i64.store
                                      local.get 1
                                      i32.const 392
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      i64.const 0
                                      i64.store
                                      local.get 1
                                      i32.const 392
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      i64.const 0
                                      i64.store
                                      local.get 1
                                      i64.const 0
                                      i64.store offset=392
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      local.get 1
                                      i32.const 392
                                      i32.add
                                      call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                                      local.get 1
                                      i32.const 480
                                      i32.add
                                      call $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE
                                      local.get 1
                                      i32.const 520
                                      i32.add
                                      i32.const 24
                                      i32.add
                                      local.get 13
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 520
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 14
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 520
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 15
                                      i64.load
                                      i64.store
                                      local.get 1
                                      local.get 1
                                      i64.load offset=480
                                      i64.store offset=520
                                      local.get 1
                                      i32.const 624
                                      i32.add
                                      i32.const 24
                                      i32.add
                                      local.get 0
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 624
                                      i32.add
                                      i32.const 16
                                      i32.add
                                      local.get 4
                                      i64.load
                                      i64.store
                                      local.get 1
                                      i32.const 624
                                      i32.add
                                      i32.const 8
                                      i32.add
                                      local.get 8
                                      i64.load
                                      i64.store
                                      local.get 1
                                      local.get 1
                                      i64.load offset=344
                                      i64.store offset=624
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      local.get 1
                                      i32.const 520
                                      i32.add
                                      local.get 1
                                      i32.const 624
                                      i32.add
                                      local.get 2
                                      local.get 3
                                      call $_ZN5psp226traits5PSP2216_approve_from_to17hc115ddc6234276ffE
                                      local.get 1
                                      i32.const 1240
                                      i32.add
                                      local.get 1
                                      i32.const 392
                                      i32.add
                                      call $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E
                                      br 9 (;@8;)
                                    end
                                    local.get 20
                                    i64.load
                                    local.set 3
                                    local.get 1
                                    i64.load offset=288
                                    local.set 2
                                    local.get 1
                                    i32.const 392
                                    i32.add
                                    i32.const 24
                                    i32.add
                                    i64.const 0
                                    i64.store
                                    local.get 1
                                    i32.const 392
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    i64.const 0
                                    i64.store
                                    local.get 1
                                    i32.const 392
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    i64.const 0
                                    i64.store
                                    local.get 1
                                    i64.const 0
                                    i64.store offset=392
                                    local.get 1
                                    i32.const 1240
                                    i32.add
                                    local.get 1
                                    i32.const 392
                                    i32.add
                                    call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                                    local.get 1
                                    i32.const 480
                                    i32.add
                                    call $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    i32.const 24
                                    i32.add
                                    local.get 13
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    local.get 14
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    local.get 15
                                    i64.load
                                    i64.store
                                    local.get 1
                                    local.get 1
                                    i64.load offset=480
                                    i64.store offset=520
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    i32.const 24
                                    i32.add
                                    local.get 0
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    local.get 4
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    local.get 8
                                    i64.load
                                    i64.store
                                    local.get 1
                                    local.get 1
                                    i64.load offset=344
                                    i64.store offset=624
                                    local.get 1
                                    i32.const 56
                                    i32.add
                                    local.get 1
                                    i32.const 1240
                                    i32.add
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    call $_ZN5psp226traits5PSP229allowance17h133c5b43bbd89b5bE
                                    local.get 1
                                    i64.load offset=56
                                    local.tee 22
                                    local.get 2
                                    i64.ge_u
                                    local.get 1
                                    i32.const 56
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    i64.load
                                    local.tee 12
                                    local.get 3
                                    i64.ge_u
                                    local.get 12
                                    local.get 3
                                    i64.eq
                                    select
                                    i32.eqz
                                    br_if 14 (;@2;)
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    i32.const 24
                                    i32.add
                                    local.get 1
                                    i32.const 480
                                    i32.add
                                    i32.const 24
                                    i32.add
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    local.get 1
                                    i32.const 480
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    local.get 1
                                    i32.const 480
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    i64.load
                                    i64.store
                                    local.get 1
                                    local.get 1
                                    i64.load offset=480
                                    i64.store offset=520
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    i32.const 24
                                    i32.add
                                    local.get 1
                                    i32.const 344
                                    i32.add
                                    i32.const 24
                                    i32.add
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    local.get 1
                                    i32.const 344
                                    i32.add
                                    i32.const 16
                                    i32.add
                                    i64.load
                                    i64.store
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    local.get 1
                                    i32.const 344
                                    i32.add
                                    i32.const 8
                                    i32.add
                                    i64.load
                                    i64.store
                                    local.get 1
                                    local.get 1
                                    i64.load offset=344
                                    i64.store offset=624
                                    local.get 1
                                    i32.const 1240
                                    i32.add
                                    local.get 1
                                    i32.const 520
                                    i32.add
                                    local.get 1
                                    i32.const 624
                                    i32.add
                                    local.get 22
                                    local.get 2
                                    i64.sub
                                    local.get 12
                                    local.get 3
                                    i64.sub
                                    local.get 22
                                    local.get 2
                                    i64.lt_u
                                    i64.extend_i32_u
                                    i64.sub
                                    call $_ZN5psp226traits5PSP2216_approve_from_to17hc115ddc6234276ffE
                                    local.get 1
                                    i32.const 1240
                                    i32.add
                                    local.get 1
                                    i32.const 392
                                    i32.add
                                    call $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E
                                    br 8 (;@8;)
                                  end
                                  local.get 20
                                  i64.load
                                  local.set 2
                                  local.get 1
                                  i64.load offset=288
                                  local.set 3
                                  local.get 1
                                  i32.const 560
                                  i32.add
                                  i32.const 24
                                  i32.add
                                  i64.const 0
                                  i64.store
                                  local.get 18
                                  i64.const 0
                                  i64.store
                                  local.get 17
                                  i64.const 0
                                  i64.store
                                  local.get 1
                                  i64.const 0
                                  i64.store offset=560
                                  local.get 1
                                  i32.const 1240
                                  i32.add
                                  local.get 1
                                  i32.const 560
                                  i32.add
                                  call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                                  local.get 1
                                  i32.const 592
                                  i32.add
                                  call $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE
                                  local.get 1
                                  i32.const 392
                                  i32.add
                                  i32.const 24
                                  i32.add
                                  local.get 1
                                  i32.const 592
                                  i32.add
                                  i32.const 24
                                  i32.add
                                  local.tee 5
                                  i64.load
                                  i64.store
                                  local.get 1
                                  i32.const 392
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.get 1
                                  i32.const 592
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.tee 9
                                  i64.load
                                  i64.store
                                  local.get 1
                                  i32.const 392
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.get 1
                                  i32.const 592
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.tee 10
                                  i64.load
                                  i64.store
                                  local.get 1
                                  local.get 1
                                  i64.load offset=592
                                  i64.store offset=392
                                  local.get 13
                                  local.get 0
                                  i64.load
                                  i64.store
                                  local.get 14
                                  local.get 4
                                  i64.load
                                  i64.store
                                  local.get 15
                                  local.get 8
                                  i64.load
                                  i64.store
                                  local.get 1
                                  local.get 1
                                  i64.load offset=344
                                  i64.store offset=480
                                  local.get 1
                                  i32.const 520
                                  i32.add
                                  i32.const 24
                                  i32.add
                                  local.get 5
                                  i64.load
                                  i64.store
                                  local.get 1
                                  i32.const 520
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.get 9
                                  i64.load
                                  i64.store
                                  local.get 1
                                  i32.const 520
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.get 10
                                  i64.load
                                  i64.store
                                  local.get 1
                                  local.get 1
                                  i64.load offset=592
                                  i64.store offset=520
                                  local.get 1
                                  i32.const 624
                                  i32.add
                                  i32.const 24
                                  i32.add
                                  local.get 0
                                  i64.load
                                  i64.store
                                  local.get 1
                                  i32.const 624
                                  i32.add
                                  i32.const 16
                                  i32.add
                                  local.get 4
                                  i64.load
                                  i64.store
                                  local.get 1
                                  i32.const 624
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  local.get 8
                                  i64.load
                                  i64.store
                                  local.get 1
                                  local.get 1
                                  i64.load offset=344
                                  i64.store offset=624
                                  local.get 1
                                  i32.const 72
                                  i32.add
                                  local.get 1
                                  i32.const 1240
                                  i32.add
                                  local.get 1
                                  i32.const 520
                                  i32.add
                                  local.get 1
                                  i32.const 624
                                  i32.add
                                  call $_ZN5psp226traits5PSP229allowance17h133c5b43bbd89b5bE
                                  local.get 3
                                  local.get 1
                                  i64.load offset=72
                                  local.tee 12
                                  i64.add
                                  local.tee 22
                                  local.get 12
                                  i64.lt_u
                                  local.tee 0
                                  local.get 2
                                  local.get 1
                                  i32.const 72
                                  i32.add
                                  i32.const 8
                                  i32.add
                                  i64.load
                                  local.tee 3
                                  i64.add
                                  local.get 0
                                  i64.extend_i32_u
                                  i64.add
                                  local.tee 2
                                  local.get 3
                                  i64.lt_u
                                  local.get 2
                                  local.get 3
                                  i64.eq
                                  select
                                  i32.const 1
                                  i32.eq
                                  br_if 13 (;@2;)
                                  local.get 1
                                  i32.const 1240
                                  i32.add
                                  local.get 1
                                  i32.const 392
                                  i32.add
                                  local.get 1
                                  i32.const 480
                                  i32.add
                                  local.get 22
                                  local.get 2
                                  call $_ZN5psp226traits5PSP2216_approve_from_to17hc115ddc6234276ffE
                                  local.get 1
                                  i32.const 1240
                                  i32.add
                                  local.get 1
                                  i32.const 560
                                  i32.add
                                  call $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E
                                  br 7 (;@8;)
                                end
                                local.get 1
                                i32.const 392
                                i32.add
                                i32.const 24
                                i32.add
                                local.get 0
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 392
                                i32.add
                                i32.const 16
                                i32.add
                                local.get 4
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 392
                                i32.add
                                i32.const 8
                                i32.add
                                local.get 8
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 392
                                i32.add
                                i32.const 34
                                i32.add
                                local.get 16
                                i32.load8_u
                                i32.store8
                                local.get 1
                                local.get 1
                                i64.load offset=344
                                i64.store offset=392
                                local.get 1
                                local.get 1
                                i32.load16_u offset=340
                                i32.store16 offset=424
                                local.get 1
                                i32.const 432
                                i32.add
                                i32.const 24
                                i32.add
                                i64.const 0
                                i64.store
                                local.get 1
                                i32.const 432
                                i32.add
                                i32.const 16
                                i32.add
                                i64.const 0
                                i64.store
                                local.get 1
                                i32.const 432
                                i32.add
                                i32.const 8
                                i32.add
                                i64.const 0
                                i64.store
                                local.get 1
                                i64.const 0
                                i64.store offset=432
                                local.get 1
                                i32.const 1240
                                i32.add
                                local.get 1
                                i32.const 432
                                i32.add
                                call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                                local.get 1
                                i32.const 480
                                i32.add
                                local.get 1
                                i32.const 392
                                i32.add
                                i32.const 35
                                call $memcpy
                                drop
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 24
                                i32.add
                                local.get 0
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 16
                                i32.add
                                local.get 4
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 8
                                i32.add
                                local.get 8
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 34
                                i32.add
                                local.get 1
                                i32.const 480
                                i32.add
                                i32.const 34
                                i32.add
                                i32.load8_u
                                i32.store8
                                local.get 1
                                local.get 1
                                i64.load offset=344
                                i64.store offset=520
                                local.get 1
                                local.get 1
                                i32.load16_u offset=512 align=1
                                i32.store16 offset=552
                                local.get 1
                                i32.const 624
                                i32.add
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 35
                                call $memcpy
                                drop
                                local.get 1
                                i32.const 560
                                i32.add
                                i32.const 24
                                i32.add
                                local.get 0
                                i64.load
                                i64.store
                                local.get 18
                                local.get 4
                                i64.load
                                i64.store
                                local.get 17
                                local.get 8
                                i64.load
                                i64.store
                                local.get 1
                                local.get 1
                                i64.load offset=344
                                i64.store offset=560
                                local.get 1
                                i32.const 607
                                i32.add
                                local.get 7
                                i64.load
                                i64.store align=1
                                local.get 1
                                i32.const 615
                                i32.add
                                local.get 6
                                i64.load
                                i64.store align=1
                                local.get 1
                                local.get 1
                                i32.const 624
                                i32.add
                                i32.const 34
                                i32.add
                                i32.load8_u
                                i32.store8 offset=594
                                local.get 1
                                local.get 1
                                i32.load16_u offset=656
                                i32.store16 offset=592
                                local.get 1
                                local.get 9
                                i32.store offset=595 align=1
                                local.get 1
                                local.get 1
                                i64.load offset=312
                                i64.store offset=599 align=1
                                local.get 1
                                local.get 10
                                i32.store8 offset=623
                                local.get 1
                                i32.const 88
                                i32.add
                                local.get 1
                                i32.const 1240
                                i32.add
                                local.get 1
                                i32.const 560
                                i32.add
                                local.get 1
                                i32.const 592
                                i32.add
                                call $_ZN5psp226traits5PSP229allowance17h133c5b43bbd89b5bE
                                local.get 1
                                local.get 1
                                i32.const 88
                                i32.add
                                i32.const 8
                                i32.add
                                i64.load
                                i64.store offset=472
                                local.get 1
                                local.get 1
                                i64.load offset=88
                                i64.store offset=464
                                local.get 1
                                i32.const 464
                                i32.add
                                call $_ZN7ink_env3api12return_value17h30fd262fc41c2a34E
                                unreachable
                              end
                              local.get 1
                              i32.const 624
                              i32.add
                              i32.const 24
                              i32.add
                              i64.const 0
                              i64.store
                              local.get 1
                              i32.const 624
                              i32.add
                              i32.const 16
                              i32.add
                              i64.const 0
                              i64.store
                              local.get 1
                              i32.const 624
                              i32.add
                              i32.const 8
                              i32.add
                              i64.const 0
                              i64.store
                              local.get 1
                              i64.const 0
                              i64.store offset=624
                              local.get 1
                              i32.const 1240
                              i32.add
                              local.get 1
                              i32.const 624
                              i32.add
                              call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                              local.get 1
                              i32.const 520
                              i32.add
                              local.get 1
                              i32.const 1712
                              i32.add
                              call $_ZN11ink_storage4lazy13Lazy$LT$T$GT$3get17haf22a627b7e19edaE
                              call $_ZN68_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17hd2312dbe62848addE
                              local.get 1
                              i32.const 520
                              i32.add
                              call $_ZN7ink_env3api12return_value17h7467eb0b235c2036E
                              unreachable
                            end
                            local.get 13
                            i64.const 0
                            i64.store
                            local.get 14
                            i64.const 0
                            i64.store
                            local.get 15
                            i64.const 0
                            i64.store
                            local.get 1
                            i64.const 0
                            i64.store offset=480
                            local.get 1
                            i32.const 1240
                            i32.add
                            local.get 1
                            i32.const 480
                            i32.add
                            call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                            block  ;; label = @13
                              local.get 1
                              i32.const 1816
                              i32.add
                              i32.load8_u
                              local.tee 0
                              i32.const 2
                              i32.ne
                              br_if 0 (;@13;)
                              i32.const 2
                              local.set 0
                              block  ;; label = @14
                                local.get 1
                                i32.const 1776
                                i32.add
                                i64.load
                                i64.const 1
                                i64.ne
                                br_if 0 (;@14;)
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 24
                                i32.add
                                local.tee 5
                                local.get 1
                                i32.const 1808
                                i32.add
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 16
                                i32.add
                                local.tee 9
                                local.get 1
                                i32.const 1800
                                i32.add
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 520
                                i32.add
                                i32.const 8
                                i32.add
                                local.tee 10
                                local.get 1
                                i32.const 1792
                                i32.add
                                i64.load
                                i64.store
                                local.get 1
                                local.get 1
                                i32.const 1784
                                i32.add
                                i64.load
                                i64.store offset=520
                                local.get 1
                                i32.const 520
                                i32.add
                                call $_ZN7ink_env3api20get_contract_storage17hed1fdc7f0b1449efE
                                i32.const 1
                                i32.and
                                local.tee 0
                                i32.eqz
                                br_if 0 (;@14;)
                                local.get 1
                                i32.const 624
                                i32.add
                                i32.const 24
                                i32.add
                                local.get 5
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 624
                                i32.add
                                i32.const 16
                                i32.add
                                local.get 9
                                i64.load
                                i64.store
                                local.get 1
                                i32.const 624
                                i32.add
                                i32.const 8
                                i32.add
                                local.get 10
                                i64.load
                                i64.store
                                local.get 1
                                local.get 1
                                i64.load offset=520
                                i64.store offset=624
                                local.get 1
                                i64.const 0
                                i64.store offset=656
                                local.get 1
                                i32.const 624
                                i32.add
                                call $_ZN11ink_storage6traits5impls5prims1_74_$LT$impl$u20$ink_storage..traits..spread..SpreadLayout$u20$for$u20$u8$GT$11pull_spread17he441713b4b29dc32E
                                local.set 4
                              end
                              local.get 1
                              i32.const 1818
                              i32.add
                              i32.const 1
                              i32.store8
                              local.get 1
                              local.get 4
                              i32.const 8
                              i32.shl
                              local.get 0
                              local.get 0
                              i32.const 2
                              i32.ne
                              i32.and
                              local.tee 0
                              i32.or
                              i32.store16 offset=1816
                            end
                            local.get 0
                            i32.const 1
                            i32.ne
                            br_if 5 (;@7;)
                            local.get 1
                            local.get 1
                            i32.load8_u offset=1817
                            i32.store8 offset=624
                            local.get 1
                            i32.const 624
                            i32.add
                            call $_ZN7ink_env3api12return_value17he3784c752d74a359E
                            unreachable
                          end
                          local.get 1
                          i32.const 624
                          i32.add
                          i32.const 24
                          i32.add
                          i64.const 0
                          i64.store
                          local.get 1
                          i32.const 624
                          i32.add
                          i32.const 16
                          i32.add
                          i64.const 0
                          i64.store
                          local.get 1
                          i32.const 624
                          i32.add
                          i32.const 8
                          i32.add
                          i64.const 0
                          i64.store
                          local.get 1
                          i64.const 0
                          i64.store offset=624
                          local.get 1
                          i32.const 1240
                          i32.add
                          local.get 1
                          i32.const 624
                          i32.add
                          call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                          local.get 1
                          i32.const 520
                          i32.add
                          local.get 1
                          i32.const 1648
                          i32.add
                          call $_ZN11ink_storage4lazy13Lazy$LT$T$GT$3get17haf22a627b7e19edaE
                          call $_ZN68_$LT$core..option..Option$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17hd2312dbe62848addE
                          local.get 1
                          i32.const 520
                          i32.add
                          call $_ZN7ink_env3api12return_value17h7467eb0b235c2036E
                          unreachable
                        end
                        local.get 1
                        i32.const 624
                        i32.add
                        i32.const 24
                        i32.add
                        i64.const 0
                        i64.store
                        local.get 1
                        i32.const 624
                        i32.add
                        i32.const 16
                        i32.add
                        i64.const 0
                        i64.store
                        local.get 1
                        i32.const 624
                        i32.add
                        i32.const 8
                        i32.add
                        i64.const 0
                        i64.store
                        local.get 1
                        i64.const 0
                        i64.store offset=624
                        local.get 1
                        i32.const 1240
                        i32.add
                        local.get 1
                        i32.const 624
                        i32.add
                        call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                        local.get 1
                        i32.const 1848
                        i32.add
                        local.get 0
                        i64.load
                        i64.store
                        local.get 1
                        i32.const 1840
                        i32.add
                        local.get 4
                        i64.load
                        i64.store
                        local.get 1
                        i32.const 1832
                        i32.add
                        local.get 8
                        i64.load
                        i64.store
                        local.get 1
                        local.get 1
                        i64.load offset=344
                        i64.store offset=1824
                        local.get 1
                        i32.const 1240
                        i32.add
                        local.get 1
                        i32.const 624
                        i32.add
                        call $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E
                        br 2 (;@8;)
                      end
                      local.get 1
                      i32.const 520
                      i32.add
                      i32.const 24
                      i32.add
                      i64.const 0
                      i64.store
                      local.get 1
                      i32.const 520
                      i32.add
                      i32.const 16
                      i32.add
                      i64.const 0
                      i64.store
                      local.get 1
                      i32.const 520
                      i32.add
                      i32.const 8
                      i32.add
                      i64.const 0
                      i64.store
                      local.get 1
                      i64.const 0
                      i64.store offset=520
                      local.get 1
                      i32.const 1240
                      i32.add
                      local.get 1
                      i32.const 520
                      i32.add
                      call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                      local.get 1
                      i32.const 624
                      i32.add
                      i32.const 24
                      i32.add
                      local.get 1
                      i32.const 1848
                      i32.add
                      i64.load
                      i64.store
                      local.get 1
                      i32.const 624
                      i32.add
                      i32.const 16
                      i32.add
                      local.get 1
                      i32.const 1840
                      i32.add
                      i64.load
                      i64.store
                      local.get 1
                      i32.const 624
                      i32.add
                      i32.const 8
                      i32.add
                      local.get 1
                      i32.const 1832
                      i32.add
                      i64.load
                      i64.store
                      local.get 1
                      local.get 1
                      i64.load offset=1824
                      i64.store offset=624
                      local.get 1
                      i32.const 624
                      i32.add
                      call $_ZN7ink_env3api12return_value17h0a13252c3ce29282E
                      unreachable
                    end
                    local.get 19
                    i64.load
                    local.set 3
                    local.get 1
                    i64.load offset=296
                    local.set 2
                    local.get 1
                    i32.const 392
                    i32.add
                    i32.const 24
                    i32.add
                    i64.const 0
                    i64.store
                    local.get 1
                    i32.const 392
                    i32.add
                    i32.const 16
                    i32.add
                    i64.const 0
                    i64.store
                    local.get 1
                    i32.const 392
                    i32.add
                    i32.const 8
                    i32.add
                    i64.const 0
                    i64.store
                    local.get 1
                    i64.const 0
                    i64.store offset=392
                    local.get 1
                    i32.const 1240
                    i32.add
                    local.get 1
                    i32.const 392
                    i32.add
                    call $_ZN11ink_storage6traits16pull_spread_root17hc3a17bf05541e6f1E
                    local.get 1
                    i32.const 480
                    i32.add
                    call $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE
                    local.get 1
                    i32.const 520
                    i32.add
                    i32.const 24
                    i32.add
                    local.get 13
                    i64.load
                    i64.store
                    local.get 1
                    i32.const 520
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 14
                    i64.load
                    i64.store
                    local.get 1
                    i32.const 520
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 15
                    i64.load
                    i64.store
                    local.get 1
                    local.get 1
                    i64.load offset=480
                    i64.store offset=520
                    local.get 1
                    i32.const 624
                    i32.add
                    i32.const 24
                    i32.add
                    local.get 0
                    i64.load
                    i64.store
                    local.get 1
                    i32.const 624
                    i32.add
                    i32.const 16
                    i32.add
                    local.get 4
                    i64.load
                    i64.store
                    local.get 1
                    i32.const 624
                    i32.add
                    i32.const 8
                    i32.add
                    local.get 8
                    i64.load
                    i64.store
                    local.get 1
                    local.get 1
                    i64.load offset=344
                    i64.store offset=624
                    local.get 1
                    local.get 9
                    i32.store offset=592
                    local.get 1
                    local.get 1
                    i64.load offset=288
                    i64.store offset=596 align=4
                    local.get 1
                    i32.const 1240
                    i32.add
                    local.get 1
                    i32.const 520
                    i32.add
                    local.get 1
                    i32.const 624
                    i32.add
                    local.get 2
                    local.get 3
                    local.get 1
                    i32.const 592
                    i32.add
                    call $_ZN5psp226traits5PSP2217_transfer_from_to17h81f2dfe081d11fc0E
                    local.get 1
                    i32.const 1240
                    i32.add
                    local.get 1
                    i32.const 392
                    i32.add
                    call $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E
                  end
                  i32.const 8
                  local.set 0
                  br 6 (;@1;)
                end
                call $_ZN4core6option13expect_failed17h076ee9a0697574d1E
                unreachable
              end
            end
            local.get 0
            i32.eqz
            br_if 1 (;@3;)
          end
          i32.const 6
          local.set 0
          br 2 (;@1;)
        end
        local.get 1
        i32.const 592
        i32.add
        i32.const 8
        i32.add
        local.tee 5
        local.get 1
        i32.const 480
        i32.add
        i32.const 8
        i32.add
        i32.load
        i32.store
        local.get 1
        i32.const 560
        i32.add
        i32.const 8
        i32.add
        local.tee 9
        local.get 1
        i32.const 392
        i32.add
        i32.const 8
        i32.add
        i32.load
        i32.store
        local.get 1
        local.get 1
        i64.load offset=480
        i64.store offset=592
        local.get 1
        local.get 1
        i64.load offset=392
        i64.store offset=560
        local.get 1
        i32.const 1288
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1296
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1240
        i32.add
        i32.const 64
        i32.add
        local.tee 0
        i32.const 0
        i32.store8
        local.get 1
        i32.const 1636
        i32.add
        i64.const 0
        i64.store align=4
        local.get 1
        i32.const 1592
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1580
        i32.add
        i64.const 0
        i64.store align=4
        local.get 1
        i32.const 1536
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1468
        i32.add
        i64.const 0
        i64.store align=4
        local.get 1
        i32.const 1424
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1412
        i32.add
        i64.const 0
        i64.store align=4
        local.get 1
        i32.const 1368
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i64.const 1
        i64.store offset=1280
        local.get 1
        i64.const 0
        i64.store offset=1240
        local.get 1
        i32.const 624
        i32.add
        call $_ZN75_$LT$ink_storage..lazy..Lazy$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h8682d878e121a73fE
        local.get 1
        i32.const 624
        i32.add
        i32.const 64
        i32.add
        call $_ZN75_$LT$ink_storage..lazy..Lazy$LT$T$GT$$u20$as$u20$core..default..Default$GT$7default17h8682d878e121a73fE
        local.get 1
        i32.const 794
        i32.add
        i32.const 0
        i32.store8
        local.get 1
        i32.const 792
        i32.add
        i32.const 1
        i32.store16
        local.get 1
        i32.const 1832
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1840
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1848
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1528
        i32.add
        i32.const 0
        i32.store
        local.get 1
        i32.const 1520
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1360
        i32.add
        i32.const 0
        i32.store
        local.get 1
        i32.const 1352
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i64.const 0
        i64.store offset=752
        local.get 1
        i64.const 0
        i64.store offset=1824
        local.get 1
        i64.const 0
        i64.store offset=1480
        local.get 1
        i64.const 0
        i64.store offset=1312
        local.get 1
        i32.const 1648
        i32.add
        local.get 1
        i32.const 624
        i32.add
        i32.const 176
        call $memcpy
        local.set 10
        local.get 1
        i32.const 624
        i32.add
        i32.const 8
        i32.add
        local.tee 11
        local.get 5
        i32.load
        i32.store
        local.get 1
        local.get 1
        i64.load offset=592
        i64.store offset=624
        local.get 10
        local.get 1
        i32.const 624
        i32.add
        call $_ZN11ink_storage4lazy13Lazy$LT$T$GT$3set17hc4bdacba33cb93cdE
        local.get 11
        local.get 9
        i32.load
        i32.store
        local.get 1
        local.get 1
        i64.load offset=560
        i64.store offset=624
        local.get 1
        i32.const 1712
        i32.add
        local.get 1
        i32.const 624
        i32.add
        call $_ZN11ink_storage4lazy13Lazy$LT$T$GT$3set17hc4bdacba33cb93cdE
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 1816
            i32.add
            i32.load8_u
            i32.const 2
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 0
            i32.store8 offset=1818
            local.get 1
            local.get 4
            i32.store8 offset=1817
            local.get 1
            i32.const 1
            i32.store8 offset=1816
            br 1 (;@3;)
          end
          local.get 1
          i32.const 1818
          i32.add
          i32.const 0
          i32.store8
          local.get 1
          local.get 4
          i32.const 8
          i32.shl
          i32.const 1
          i32.or
          i32.store16 offset=1816
        end
        local.get 1
        i32.const 520
        i32.add
        call $_ZN8ink_lang10env_access18EnvAccess$LT$T$GT$6caller17hadba7c3571d7e51fE
        local.get 1
        i32.const 520
        i32.add
        call $_ZN140_$LT$$LT$ink_env..types..DefaultEnvironment$u20$as$u20$ink_env..types..Environment$GT$..AccountId$u20$as$u20$brush..traits..AccountIdExt$GT$7is_zero17hd36245406920208bE
        br_if 0 (;@2;)
        local.get 1
        i32.const 624
        i32.add
        i32.const 24
        i32.add
        local.tee 5
        local.get 1
        i32.const 520
        i32.add
        i32.const 24
        i32.add
        local.tee 9
        i64.load
        i64.store
        local.get 1
        i32.const 624
        i32.add
        i32.const 16
        i32.add
        local.tee 10
        local.get 1
        i32.const 520
        i32.add
        i32.const 16
        i32.add
        local.tee 11
        i64.load
        i64.store
        local.get 1
        i32.const 624
        i32.add
        i32.const 8
        i32.add
        local.tee 8
        local.get 1
        i32.const 520
        i32.add
        i32.const 8
        i32.add
        local.tee 7
        i64.load
        i64.store
        local.get 1
        local.get 1
        i64.load offset=520
        i64.store offset=624
        local.get 1
        i32.const 240
        i32.add
        local.get 1
        i32.const 1240
        i32.add
        local.get 1
        i32.const 624
        i32.add
        call $_ZN5psp226traits5PSP2210balance_of17h8e8fb26b27704ba9E
        local.get 1
        i64.load offset=240
        local.tee 12
        local.get 3
        i64.add
        local.tee 21
        local.get 12
        i64.lt_u
        local.tee 4
        local.get 1
        i32.const 240
        i32.add
        i32.const 8
        i32.add
        i64.load
        local.tee 12
        local.get 2
        i64.add
        local.get 4
        i64.extend_i32_u
        i64.add
        local.tee 22
        local.get 12
        i64.lt_u
        local.get 22
        local.get 12
        i64.eq
        select
        br_if 0 (;@2;)
        local.get 5
        local.get 9
        i64.load
        i64.store
        local.get 10
        local.get 11
        i64.load
        i64.store
        local.get 8
        local.get 7
        i64.load
        i64.store
        local.get 1
        local.get 1
        i64.load offset=520
        i64.store offset=624
        local.get 1
        i32.const 1312
        i32.add
        local.get 1
        i32.const 624
        i32.add
        local.get 21
        local.get 22
        call $_ZN11ink_storage11collections7hashmap24HashMap$LT$K$C$V$C$H$GT$6insert17hd1bfc3a3e6a0334aE
        local.get 1
        i32.const 224
        i32.add
        local.get 1
        i32.const 1240
        i32.add
        call $_ZN5psp226traits5PSP2212total_supply17h3afc31b33677089bE
        local.get 1
        i64.load offset=224
        local.tee 12
        local.get 3
        i64.add
        local.tee 22
        local.get 12
        i64.lt_u
        local.tee 4
        local.get 1
        i32.const 224
        i32.add
        i32.const 8
        i32.add
        i64.load
        local.tee 3
        local.get 2
        i64.add
        local.get 4
        i64.extend_i32_u
        i64.add
        local.tee 2
        local.get 3
        i64.lt_u
        local.get 2
        local.get 3
        i64.eq
        select
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i64.load offset=1280
            i64.const 2
            i64.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 1296
            i32.add
            local.get 2
            i64.store
            local.get 1
            local.get 22
            i64.store offset=1288
            local.get 1
            i64.const 1
            i64.store offset=1280
            local.get 1
            i32.const 1280
            i32.add
            i32.const 24
            i32.add
            local.set 0
            br 1 (;@3;)
          end
          local.get 1
          i32.const 1296
          i32.add
          local.get 2
          i64.store
          local.get 1
          local.get 22
          i64.store offset=1288
          local.get 1
          i64.const 1
          i64.store offset=1280
        end
        local.get 0
        i32.const 0
        i32.store8
        local.get 1
        i32.const 624
        i32.add
        local.get 1
        i32.const 1240
        i32.add
        i32.const 616
        call $memcpy
        drop
        local.get 1
        i32.const 1264
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i32.const 1256
        i32.add
        i64.const 0
        i64.store
        i32.const 8
        local.set 0
        local.get 1
        i32.const 1240
        i32.add
        i32.const 8
        i32.add
        i64.const 0
        i64.store
        local.get 1
        i64.const 0
        i64.store offset=1240
        local.get 1
        i32.const 624
        i32.add
        local.get 1
        i32.const 1240
        i32.add
        call $_ZN11ink_storage6traits16push_spread_root17h25859b4456733671E
        br 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 1
    i32.const 1856
    i32.add
    global.set 0
    local.get 0)
  (func $call (type 15) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 16384
    i32.store offset=36
    local.get 0
    i32.const 68528
    i32.store offset=32
    local.get 0
    i32.const 16384
    i32.store offset=40
    i32.const 68528
    local.get 0
    i32.const 40
    i32.add
    call $_ZN7ink_env6engine8on_chain3ext3sys22seal_value_transferred17h916ed8215b2046a7E
    local.get 0
    i32.const 32
    i32.add
    local.get 0
    i32.load offset=40
    call $_ZN7ink_env6engine8on_chain3ext18extract_from_slice17h8e422b55d0a95429E
    local.get 0
    local.get 0
    i64.load offset=32
    i64.store offset=40
    local.get 0
    i32.const 8
    i32.add
    local.get 0
    i32.const 40
    i32.add
    call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
    block  ;; label = @1
      local.get 0
      i64.load offset=8
      i32.wrap_i64
      br_if 0 (;@1;)
      local.get 0
      i64.load offset=16
      local.get 0
      i32.const 24
      i32.add
      i64.load
      i64.or
      i64.eqz
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1
      call $_ZN8my_psp228my_psp221_95_$LT$impl$u20$ink_lang..contract..DispatchUsingMode$u20$for$u20$my_psp22..my_psp22..MyPSP22$GT$19dispatch_using_mode17h80c60cbc72520667E
      local.set 1
      local.get 0
      i32.const 48
      i32.add
      global.set 0
      local.get 1
      i32.const 255
      i32.and
      i32.const 2
      i32.shl
      i32.const 65700
      i32.add
      i32.load
      return
    end
    call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
    unreachable)
  (func $_ZN7ink_env6engine8on_chain3ext5input17h026d86c7dec708cfE (type 5) (param i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    i32.load offset=4
    i32.store offset=12
    local.get 0
    i32.load
    local.get 1
    i32.const 12
    i32.add
    call $_ZN7ink_env6engine8on_chain3ext3sys10seal_input17h7fb0017c5c621ea9E
    local.get 0
    local.get 1
    i32.load offset=12
    call $_ZN7ink_env6engine8on_chain3ext18extract_from_slice17h8e422b55d0a95429E
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN78_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17hf1fcdcd6d2b09f13E (type 4) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 24
    i32.add
    local.get 1
    call $_ZN18parity_scale_codec5codec5Input9read_byte17h9c00b7cc60b881a3E
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.load8_u offset=24
        i32.const 1
        i32.and
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 2
                  i32.load8_u offset=25
                  local.tee 3
                  i32.const 3
                  i32.and
                  local.tee 4
                  i32.const 3
                  i32.eq
                  br_if 0 (;@7;)
                  local.get 4
                  br_table 1 (;@6;) 2 (;@5;) 3 (;@4;) 1 (;@6;)
                end
                local.get 3
                i32.const 255
                i32.and
                i32.const 4
                i32.ge_u
                br_if 4 (;@2;)
                local.get 2
                i32.const 16
                i32.add
                local.get 1
                call $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E
                local.get 2
                i32.load offset=16
                br_if 4 (;@2;)
                local.get 2
                i32.load offset=20
                local.tee 4
                i32.const 1073741823
                i32.le_u
                br_if 4 (;@2;)
                br 3 (;@3;)
              end
              local.get 3
              i32.const 252
              i32.and
              i32.const 2
              i32.shr_u
              local.set 4
              br 2 (;@3;)
            end
            local.get 2
            local.get 3
            i32.store8 offset=37
            local.get 2
            i32.const 1
            i32.store8 offset=36
            local.get 2
            local.get 1
            i32.store offset=32
            local.get 2
            i32.const 0
            i32.store16 offset=44
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.const 32
                i32.add
                local.get 2
                i32.const 44
                i32.add
                i32.const 2
                call $_ZN102_$LT$parity_scale_codec..compact..PrefixInput$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17hc0140882c03a8a52E
                local.tee 5
                br_if 0 (;@6;)
                local.get 2
                i32.load16_u offset=44
                local.set 4
                br 1 (;@5;)
              end
            end
            i32.const 0
            local.set 3
            local.get 5
            br_if 3 (;@1;)
            local.get 4
            i32.const 65535
            i32.and
            i32.const 255
            i32.le_u
            br_if 3 (;@1;)
            local.get 4
            i32.const 65532
            i32.and
            i32.const 2
            i32.shr_u
            local.set 4
            br 1 (;@3;)
          end
          local.get 2
          local.get 3
          i32.store8 offset=37
          local.get 2
          i32.const 1
          i32.store8 offset=36
          local.get 2
          local.get 1
          i32.store offset=32
          i32.const 0
          local.set 3
          local.get 2
          i32.const 0
          i32.store offset=44
          local.get 2
          i32.const 32
          i32.add
          local.get 2
          i32.const 44
          i32.add
          i32.const 4
          call $_ZN102_$LT$parity_scale_codec..compact..PrefixInput$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17hc0140882c03a8a52E
          br_if 2 (;@1;)
          local.get 2
          i32.load offset=44
          local.tee 4
          i32.const 65536
          i32.lt_u
          br_if 2 (;@1;)
          local.get 4
          i32.const 2
          i32.shr_u
          local.set 4
        end
        local.get 1
        i32.load offset=4
        local.get 4
        i32.lt_u
        br_if 0 (;@2;)
        local.get 2
        i32.const 8
        i32.add
        local.get 4
        i32.const 1
        call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17h90fe4a90cd55eb2fE
        local.get 2
        i32.load offset=12
        local.set 6
        i32.const 0
        local.set 3
        local.get 1
        local.get 2
        i32.load offset=8
        local.tee 5
        local.get 4
        call $_ZN69_$LT$$RF$$u5b$u8$u5d$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17ha43a819276410a89E
        br_if 1 (;@1;)
        block  ;; label = @3
          local.get 5
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          i64.extend_i32_u
          i64.const 32
          i64.shl
          local.get 6
          i64.extend_i32_u
          i64.or
          i64.store offset=4 align=4
        end
        local.get 5
        local.set 3
        br 1 (;@1;)
      end
      i32.const 0
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func $_ZN4core3fmt5Write9write_fmt17h147d222b45c4182cE (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 65736
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17ha77d2333e4c6ece8E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt5write17ha77d2333e4c6ece8E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 52
    i32.add
    local.get 1
    i32.store
    local.get 3
    i32.const 3
    i32.store8 offset=56
    local.get 3
    i64.const 137438953472
    i64.store offset=24
    local.get 3
    local.get 0
    i32.store offset=48
    i32.const 0
    local.set 1
    local.get 3
    i32.const 0
    i32.store offset=40
    local.get 3
    i32.const 0
    i32.store offset=32
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load offset=8
            local.tee 0
            br_if 0 (;@4;)
            local.get 2
            i32.const 20
            i32.add
            i32.load
            i32.const 536870911
            i32.and
            local.tee 4
            i32.const 1
            i32.add
            local.set 5
            local.get 2
            i32.load offset=16
            local.set 6
            i32.const 0
            local.set 0
            loop  ;; label = @5
              local.get 5
              i32.const -1
              i32.add
              local.tee 5
              i32.eqz
              br_if 2 (;@3;)
              block  ;; label = @6
                local.get 2
                i32.load
                local.get 0
                i32.add
                local.tee 1
                i32.const 4
                i32.add
                i32.load
                local.tee 7
                i32.eqz
                br_if 0 (;@6;)
                local.get 3
                i32.load offset=48
                local.get 1
                i32.load
                local.get 7
                local.get 3
                i32.load offset=52
                i32.load offset=12
                call_indirect (type 0)
                br_if 4 (;@2;)
              end
              local.get 6
              local.get 0
              i32.add
              local.set 1
              local.get 0
              i32.const 8
              i32.add
              local.set 0
              local.get 1
              i32.load
              local.get 3
              i32.const 24
              i32.add
              local.get 1
              i32.const 4
              i32.add
              i32.load
              call_indirect (type 1)
              i32.eqz
              br_if 0 (;@5;)
              br 3 (;@2;)
            end
          end
          local.get 2
          i32.const 12
          i32.add
          i32.load
          local.tee 7
          i32.const 5
          i32.shl
          local.set 5
          local.get 7
          i32.const 134217727
          i32.and
          local.set 4
          loop  ;; label = @4
            local.get 5
            i32.eqz
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 2
              i32.load
              local.get 1
              i32.add
              local.tee 7
              i32.const 4
              i32.add
              i32.load
              local.tee 6
              i32.eqz
              br_if 0 (;@5;)
              local.get 3
              i32.load offset=48
              local.get 7
              i32.load
              local.get 6
              local.get 3
              i32.load offset=52
              i32.load offset=12
              call_indirect (type 0)
              br_if 3 (;@2;)
            end
            local.get 3
            local.get 0
            i32.load8_u offset=28
            i32.store8 offset=56
            local.get 3
            local.get 0
            i64.load offset=4 align=4
            i64.const 32
            i64.rotl
            i64.store offset=24
            local.get 3
            i32.const 16
            i32.add
            local.get 2
            i32.load offset=16
            local.tee 7
            local.get 0
            i32.const 20
            i32.add
            call $_ZN4core3fmt8getcount17h1818d9452d192dbcE
            local.get 3
            local.get 3
            i64.load offset=16
            i64.store offset=32
            local.get 3
            i32.const 8
            i32.add
            local.get 7
            local.get 0
            i32.const 12
            i32.add
            call $_ZN4core3fmt8getcount17h1818d9452d192dbcE
            local.get 3
            local.get 3
            i64.load offset=8
            i64.store offset=40
            local.get 1
            i32.const 8
            i32.add
            local.set 1
            local.get 5
            i32.const -32
            i32.add
            local.set 5
            local.get 0
            i32.load
            local.set 6
            local.get 0
            i32.const 32
            i32.add
            local.set 0
            local.get 7
            local.get 6
            i32.const 3
            i32.shl
            i32.add
            local.tee 7
            i32.load
            local.get 3
            i32.const 24
            i32.add
            local.get 7
            i32.load offset=4
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
            br 2 (;@2;)
          end
        end
        i32.const 0
        local.set 0
        local.get 4
        local.get 2
        i32.load offset=4
        i32.lt_u
        local.tee 1
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=48
        local.get 2
        i32.load
        local.get 4
        i32.const 3
        i32.shl
        i32.add
        i32.const 0
        local.get 1
        select
        local.tee 2
        i32.load
        local.get 2
        i32.load offset=4
        local.get 3
        i32.load offset=52
        i32.load offset=12
        call_indirect (type 0)
        i32.eqz
        br_if 1 (;@1;)
      end
      i32.const 1
      local.set 0
    end
    local.get 3
    i32.const 64
    i32.add
    global.set 0
    local.get 0)
  (func $_ZN4core3ptr54drop_in_place$LT$$RF$mut$u20$alloc..string..String$GT$17h6d097882788d66fbE (type 5) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hfb52867266a9c081E (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $_ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17hd23f23b49dcc7969E
    i32.const 0)
  (func $_ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17hd23f23b49dcc7969E (type 2) (param i32 i32 i32)
    (local i32)
    local.get 0
    local.get 2
    call $_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h2275f384fce89919E
    local.get 0
    i32.load
    local.get 0
    i32.load offset=8
    local.tee 3
    i32.add
    local.get 1
    local.get 2
    call $memcpy
    drop
    block  ;; label = @1
      local.get 3
      local.get 2
      i32.add
      local.tee 2
      local.get 3
      i32.ge_u
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=8)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h486b065076821b1bE (type 1) (param i32 i32) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  local.get 1
                  i32.const 127
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 0
                  i32.load offset=8
                  local.tee 3
                  local.get 0
                  i32.load offset=4
                  i32.eq
                  br_if 1 (;@6;)
                  br 4 (;@3;)
                end
                local.get 2
                i32.const 0
                i32.store offset=12
                local.get 1
                i32.const 2048
                i32.lt_u
                br_if 1 (;@5;)
                block  ;; label = @7
                  local.get 1
                  i32.const 65536
                  i32.ge_u
                  br_if 0 (;@7;)
                  local.get 2
                  local.get 1
                  i32.const 63
                  i32.and
                  i32.const 128
                  i32.or
                  i32.store8 offset=14
                  local.get 2
                  local.get 1
                  i32.const 12
                  i32.shr_u
                  i32.const 224
                  i32.or
                  i32.store8 offset=12
                  local.get 2
                  local.get 1
                  i32.const 6
                  i32.shr_u
                  i32.const 63
                  i32.and
                  i32.const 128
                  i32.or
                  i32.store8 offset=13
                  i32.const 3
                  local.set 1
                  br 3 (;@4;)
                end
                local.get 2
                local.get 1
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=15
                local.get 2
                local.get 1
                i32.const 18
                i32.shr_u
                i32.const 240
                i32.or
                i32.store8 offset=12
                local.get 2
                local.get 1
                i32.const 6
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=14
                local.get 2
                local.get 1
                i32.const 12
                i32.shr_u
                i32.const 63
                i32.and
                i32.const 128
                i32.or
                i32.store8 offset=13
                i32.const 4
                local.set 1
                br 2 (;@4;)
              end
              local.get 0
              i32.const 1
              call $_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h2275f384fce89919E
              local.get 0
              i32.load offset=8
              local.set 3
              br 2 (;@3;)
            end
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 192
            i32.or
            i32.store8 offset=12
            i32.const 2
            local.set 1
          end
          local.get 0
          local.get 2
          i32.const 12
          i32.add
          local.get 1
          call $_ZN5alloc3vec16Vec$LT$T$C$A$GT$17extend_from_slice17hd23f23b49dcc7969E
          br 1 (;@2;)
        end
        local.get 0
        i32.load
        local.get 3
        i32.add
        local.get 1
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.tee 1
        local.get 3
        i32.lt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        i32.store offset=8
      end
      local.get 2
      i32.const 16
      i32.add
      global.set 0
      i32.const 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc3vec16Vec$LT$T$C$A$GT$7reserve17h2275f384fce89919E (type 4) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load offset=4
        local.tee 3
        local.get 0
        i32.load offset=8
        local.tee 4
        i32.sub
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 4
        local.get 1
        i32.add
        local.tee 1
        local.get 4
        i32.lt_u
        br_if 1 (;@1;)
        local.get 3
        local.get 3
        i32.add
        local.tee 4
        local.get 3
        i32.lt_u
        br_if 1 (;@1;)
        local.get 2
        local.get 4
        local.get 1
        local.get 4
        local.get 1
        i32.gt_u
        select
        local.tee 1
        i32.const 8
        local.get 1
        i32.const 8
        i32.gt_u
        select
        local.get 0
        i32.load
        i32.const 0
        local.get 3
        select
        local.get 3
        i32.const 1
        call $_ZN5alloc7raw_vec11finish_grow17h8457fc0e92463552E
        local.get 2
        i32.load
        i32.const 1
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        local.get 2
        i64.load offset=4 align=4
        i64.store align=4
      end
      local.get 2
      i32.const 16
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h8ae99549e2ec2661E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 0
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5Write9write_fmt17h147d222b45c4182cE
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN5alloc7raw_vec11finish_grow17h8457fc0e92463552E (type 16) (param i32 i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 5
    global.set 0
    i32.const 0
    local.set 6
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                local.get 2
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  local.get 3
                  br_if 0 (;@7;)
                  local.get 5
                  i32.const 8
                  i32.add
                  local.get 1
                  i32.const 0
                  call $_ZN5alloc5alloc6Global10alloc_impl17h0e8393935665a52eE
                  local.get 5
                  i32.load offset=12
                  local.set 6
                  local.get 5
                  i32.load offset=8
                  local.set 7
                  br 4 (;@3;)
                end
                i32.const 0
                local.set 7
                local.get 1
                local.set 6
                i32.const 0
                i32.load offset=68520
                local.tee 8
                local.get 1
                i32.add
                local.tee 9
                local.get 8
                i32.lt_u
                br_if 3 (;@3;)
                block  ;; label = @7
                  local.get 9
                  i32.const 0
                  i32.load offset=68524
                  i32.le_u
                  br_if 0 (;@7;)
                  local.get 1
                  i32.const 65536
                  i32.add
                  local.tee 6
                  local.get 1
                  i32.lt_u
                  br_if 2 (;@5;)
                  local.get 6
                  i32.const -1
                  i32.add
                  local.tee 7
                  i32.const 16
                  i32.shr_u
                  memory.grow
                  local.tee 6
                  i32.const -1
                  i32.eq
                  br_if 3 (;@4;)
                  local.get 6
                  i32.const 65535
                  i32.and
                  local.get 6
                  i32.ne
                  br_if 3 (;@4;)
                  local.get 6
                  i32.const 16
                  i32.shl
                  local.tee 8
                  local.get 7
                  i32.const -65536
                  i32.and
                  i32.add
                  local.tee 6
                  local.get 8
                  i32.lt_u
                  br_if 3 (;@4;)
                  i32.const 0
                  local.set 7
                  i32.const 0
                  local.get 6
                  i32.store offset=68524
                  local.get 1
                  local.set 6
                  local.get 8
                  local.get 1
                  i32.add
                  local.tee 9
                  local.get 8
                  i32.lt_u
                  br_if 4 (;@3;)
                end
                i32.const 0
                local.get 9
                i32.store offset=68520
                local.get 1
                local.set 6
                i32.const 0
                local.set 7
                local.get 8
                i32.eqz
                br_if 3 (;@3;)
                local.get 1
                local.set 6
                local.get 8
                local.get 2
                local.get 3
                call $memcpy
                local.set 7
                br 3 (;@3;)
              end
              local.get 5
              local.get 1
              call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE
              local.get 5
              i32.load offset=4
              local.set 6
              local.get 5
              i32.load
              local.set 7
              br 2 (;@3;)
            end
            unreachable
            unreachable
          end
          local.get 1
          local.set 6
          i32.const 0
          local.set 7
        end
        block  ;; label = @3
          local.get 7
          i32.eqz
          br_if 0 (;@3;)
          local.get 0
          local.get 7
          i32.store offset=4
          i32.const 0
          local.set 1
          br 2 (;@1;)
        end
        local.get 0
        local.get 1
        i32.store offset=4
        i32.const 1
        local.set 6
      end
      i32.const 1
      local.set 1
    end
    local.get 0
    local.get 1
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 6
    i32.store
    local.get 5
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5alloc5alloc6Global10alloc_impl17h0e8393935665a52eE (type 2) (param i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          br_if 0 (;@3;)
          i32.const 1
          local.set 2
          br 1 (;@2;)
        end
        block  ;; label = @3
          local.get 2
          br_if 0 (;@3;)
          local.get 1
          i32.const 1
          call $_ZN87_$LT$ink_allocator..bump..BumpAllocator$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h3b5b87aeed817ef7E
          local.set 2
          br 1 (;@2;)
        end
        i32.const 0
        local.set 2
        i32.const 0
        i32.load offset=68520
        local.tee 3
        local.get 1
        i32.add
        local.tee 4
        local.get 3
        i32.lt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 4
          i32.const 0
          i32.load offset=68524
          i32.le_u
          br_if 0 (;@3;)
          local.get 1
          i32.const 65536
          i32.add
          local.tee 3
          local.get 1
          i32.lt_u
          br_if 2 (;@1;)
          i32.const 0
          local.set 2
          local.get 3
          i32.const -1
          i32.add
          local.tee 4
          i32.const 16
          i32.shr_u
          memory.grow
          local.tee 3
          i32.const -1
          i32.eq
          br_if 1 (;@2;)
          local.get 3
          i32.const 65535
          i32.and
          local.get 3
          i32.ne
          br_if 1 (;@2;)
          local.get 3
          i32.const 16
          i32.shl
          local.tee 3
          local.get 4
          i32.const -65536
          i32.and
          i32.add
          local.tee 4
          local.get 3
          i32.lt_u
          br_if 1 (;@2;)
          i32.const 0
          local.set 2
          i32.const 0
          local.get 4
          i32.store offset=68524
          local.get 3
          local.get 1
          i32.add
          local.tee 4
          local.get 3
          i32.lt_u
          br_if 1 (;@2;)
        end
        i32.const 0
        local.get 4
        i32.store offset=68520
        local.get 3
        local.set 2
      end
      local.get 0
      local.get 1
      i32.store offset=4
      local.get 0
      local.get 2
      i32.store
      return
    end
    unreachable
    unreachable)
  (func $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 0
    call $_ZN5alloc5alloc6Global10alloc_impl17h0e8393935665a52eE
    local.get 0
    local.get 2
    i32.load offset=8
    i32.store
    local.get 0
    local.get 2
    i32.load offset=12
    i32.store offset=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$16with_capacity_in17h55f2c99e227cc199E (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    i32.const 0
    call $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17h90fe4a90cd55eb2fE
    local.get 0
    local.get 2
    i32.load offset=8
    i32.store
    local.get 0
    local.get 2
    i32.load offset=12
    i32.store offset=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in17h90fe4a90cd55eb2fE (type 2) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 0
        i32.lt_s
        br_if 0 (;@2;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            br_if 0 (;@4;)
            local.get 3
            i32.const 8
            i32.add
            local.get 1
            call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE
            local.get 3
            i32.load offset=12
            local.set 2
            local.get 3
            i32.load offset=8
            local.set 1
            br 1 (;@3;)
          end
          local.get 3
          local.get 1
          i32.const 1
          call $_ZN5alloc5alloc6Global10alloc_impl17h0e8393935665a52eE
          local.get 3
          i32.load offset=4
          local.set 2
          local.get 3
          i32.load
          local.set 1
        end
        local.get 1
        br_if 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 0
    local.get 1
    i32.store
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5alloc11collections5btree4node10splitpoint17hf3cba376e88c5856E (type 4) (param i32 i32)
    (local i32 i32)
    i32.const 0
    local.set 2
    i32.const 4
    local.set 3
    block  ;; label = @1
      local.get 1
      i32.const 5
      i32.lt_u
      br_if 0 (;@1;)
      local.get 1
      local.set 3
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.const -5
          i32.add
          br_table 2 (;@1;) 0 (;@3;) 1 (;@2;)
        end
        i32.const 0
        local.set 1
        i32.const 1
        local.set 2
        i32.const 5
        local.set 3
        br 1 (;@1;)
      end
      local.get 1
      i32.const -7
      i32.add
      local.set 1
      i32.const 1
      local.set 2
      i32.const 6
      local.set 3
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    i32.store)
  (func $_ZN4core3ops8function6FnOnce9call_once17h01f7e944991b7a5dE (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    drop
    loop (result i32)  ;; label = @1
      br 0 (;@1;)
    end)
  (func $_ZN4core5slice5index26slice_start_index_len_fail17h71cc3842941338daE (type 8)
    unreachable
    unreachable)
  (func $_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h8df044a0ee528d6bE (type 16) (param i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.gt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 2
          local.get 4
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i32.sub
          local.tee 4
          local.get 2
          i32.gt_u
          br_if 2 (;@1;)
          local.get 0
          local.get 4
          i32.store offset=4
          local.get 0
          local.get 3
          local.get 1
          i32.add
          i32.store
          return
        end
        call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
        unreachable
      end
      call $_ZN4core5slice5index22slice_index_order_fail17h1824dbaa9030e48aE
      unreachable
    end
    unreachable
    unreachable)
  (func $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE (type 8)
    unreachable
    unreachable)
  (func $_ZN4core5slice5index22slice_index_order_fail17h1824dbaa9030e48aE (type 8)
    unreachable
    unreachable)
  (func $_ZN4core3fmt9Formatter3pad17h9ed4c84fd538e72aE (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 0
    i32.load offset=16
    local.set 4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 0
            i32.load offset=8
            local.tee 5
            i32.const 1
            i32.eq
            br_if 0 (;@4;)
            local.get 4
            br_if 1 (;@3;)
            local.get 0
            i32.load offset=24
            local.get 1
            local.get 2
            local.get 0
            i32.const 28
            i32.add
            i32.load
            i32.load offset=12
            call_indirect (type 0)
            local.set 4
            br 3 (;@1;)
          end
          local.get 4
          i32.eqz
          br_if 1 (;@2;)
        end
        local.get 0
        i32.const 20
        i32.add
        i32.load
        local.set 4
        local.get 3
        local.get 1
        i32.store offset=36
        local.get 3
        i32.const 40
        i32.add
        local.get 1
        local.get 2
        i32.add
        i32.store
        local.get 3
        i32.const 0
        i32.store offset=32
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        block  ;; label = @3
          block  ;; label = @4
            loop  ;; label = @5
              local.get 4
              i32.const -1
              i32.add
              local.tee 4
              i32.eqz
              br_if 1 (;@4;)
              local.get 3
              i32.const 24
              i32.add
              local.get 3
              i32.const 32
              i32.add
              call $_ZN87_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfb31d1bde6bbf7ffE
              local.get 3
              i32.load offset=28
              i32.const 1114112
              i32.ne
              br_if 0 (;@5;)
              br 2 (;@3;)
            end
          end
          local.get 3
          i32.const 16
          i32.add
          local.get 3
          i32.const 32
          i32.add
          call $_ZN87_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfb31d1bde6bbf7ffE
          local.get 3
          i32.load offset=20
          i32.const 1114112
          i32.eq
          br_if 0 (;@3;)
          local.get 3
          i32.const 8
          i32.add
          local.get 3
          i32.load offset=16
          local.get 1
          local.get 2
          call $_ZN4core3str6traits110_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeTo$LT$usize$GT$$GT$3get17he2f9dbd78df77a2eE
          local.get 3
          i32.load offset=12
          local.get 2
          local.get 3
          i32.load offset=8
          local.tee 4
          select
          local.set 2
          local.get 4
          local.get 1
          local.get 4
          select
          local.set 1
        end
        local.get 5
        br_if 0 (;@2;)
        local.get 0
        i32.load offset=24
        local.get 1
        local.get 2
        local.get 0
        i32.const 28
        i32.add
        i32.load
        i32.load offset=12
        call_indirect (type 0)
        local.set 4
        br 1 (;@1;)
      end
      local.get 0
      i32.const 12
      i32.add
      i32.load
      local.set 6
      i32.const 0
      local.set 4
      i32.const 0
      local.set 5
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            local.get 2
            local.get 4
            i32.eq
            br_if 1 (;@3;)
            local.get 5
            local.get 1
            local.get 4
            i32.add
            i32.load8_u
            i32.const 192
            i32.and
            i32.const 128
            i32.ne
            i32.add
            local.tee 7
            local.get 5
            i32.lt_u
            br_if 2 (;@2;)
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 7
            local.set 5
            br 0 (;@4;)
          end
        end
        block  ;; label = @3
          local.get 6
          local.get 5
          i32.gt_u
          br_if 0 (;@3;)
          local.get 0
          i32.load offset=24
          local.get 1
          local.get 2
          local.get 0
          i32.const 28
          i32.add
          i32.load
          i32.load offset=12
          call_indirect (type 0)
          local.set 4
          br 2 (;@1;)
        end
        i32.const 0
        local.set 4
        local.get 6
        local.get 5
        i32.sub
        local.tee 5
        local.set 6
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              i32.const 0
              local.get 0
              i32.load8_u offset=32
              local.tee 7
              local.get 7
              i32.const 3
              i32.eq
              select
              i32.const 3
              i32.and
              br_table 2 (;@3;) 1 (;@4;) 0 (;@5;) 1 (;@4;) 2 (;@3;)
            end
            local.get 5
            i32.const 1
            i32.add
            local.tee 4
            local.get 5
            i32.lt_u
            br_if 2 (;@2;)
            local.get 4
            i32.const 1
            i32.shr_u
            local.set 6
            local.get 5
            i32.const 1
            i32.shr_u
            local.set 4
            br 1 (;@3;)
          end
          i32.const 0
          local.set 6
          local.get 5
          local.set 4
        end
        local.get 4
        i32.const 1
        i32.add
        local.set 4
        local.get 0
        i32.const 28
        i32.add
        i32.load
        local.set 7
        local.get 0
        i32.load offset=4
        local.set 5
        local.get 0
        i32.load offset=24
        local.set 0
        block  ;; label = @3
          loop  ;; label = @4
            local.get 4
            i32.const -1
            i32.add
            local.tee 4
            i32.eqz
            br_if 1 (;@3;)
            local.get 0
            local.get 5
            local.get 7
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
          end
          i32.const 1
          local.set 4
          br 2 (;@1;)
        end
        i32.const 1
        local.set 4
        local.get 5
        i32.const 1114112
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        local.get 1
        local.get 2
        local.get 7
        i32.load offset=12
        call_indirect (type 0)
        br_if 1 (;@1;)
        i32.const 0
        local.set 4
        block  ;; label = @3
          loop  ;; label = @4
            block  ;; label = @5
              local.get 6
              local.get 4
              i32.ne
              br_if 0 (;@5;)
              local.get 6
              local.set 4
              br 2 (;@3;)
            end
            local.get 4
            i32.const 1
            i32.add
            local.set 4
            local.get 0
            local.get 5
            local.get 7
            i32.load offset=16
            call_indirect (type 1)
            i32.eqz
            br_if 0 (;@4;)
          end
          local.get 4
          i32.const -1
          i32.add
          local.set 4
        end
        local.get 4
        local.get 6
        i32.lt_u
        local.set 4
        br 1 (;@1;)
      end
      unreachable
      unreachable
    end
    local.get 3
    i32.const 48
    i32.add
    global.set 0
    local.get 4)
  (func $_ZN87_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfb31d1bde6bbf7ffE (type 4) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    i32.const 8
    i32.add
    i32.load
    local.set 2
    local.get 1
    i32.load offset=4
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.const 4
        i32.add
        call $_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h36ebcf67f1b3ee3fE.34
        local.tee 4
        i32.const 1114112
        i32.eq
        br_if 0 (;@2;)
        local.get 2
        local.get 3
        i32.sub
        local.tee 5
        local.get 1
        i32.load offset=4
        local.get 1
        i32.load offset=8
        i32.sub
        i32.add
        local.tee 2
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 1
        i32.load
        local.tee 5
        local.get 2
        i32.add
        local.tee 2
        local.get 5
        i32.lt_u
        br_if 1 (;@1;)
        local.get 1
        local.get 2
        i32.store
      end
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      local.get 5
      i32.store
      return
    end
    unreachable
    unreachable)
  (func $_ZN4core3str6traits110_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeTo$LT$usize$GT$$GT$3get17he2f9dbd78df77a2eE (type 13) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
          i32.const 0
          local.set 4
          local.get 3
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        i32.const 0
        local.set 4
        local.get 2
        local.get 1
        i32.add
        i32.load8_s
        i32.const -64
        i32.lt_s
        br_if 1 (;@1;)
      end
      local.get 2
      local.set 4
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 4
    i32.store)
  (func $_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h36ebcf67f1b3ee3fE.34 (type 7) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      local.tee 1
      local.get 0
      i32.load offset=4
      local.tee 2
      i32.ne
      br_if 0 (;@1;)
      i32.const 1114112
      return
    end
    local.get 0
    local.get 1
    i32.const 1
    i32.add
    local.tee 3
    i32.store
    block  ;; label = @1
      local.get 1
      i32.load8_u
      local.tee 4
      i32.const 24
      i32.shl
      i32.const 24
      i32.shr_s
      i32.const -1
      i32.gt_s
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 2
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          local.set 5
          local.get 2
          local.set 3
          br 1 (;@2;)
        end
        local.get 0
        local.get 1
        i32.const 2
        i32.add
        local.tee 3
        i32.store
        local.get 1
        i32.load8_u offset=1
        i32.const 63
        i32.and
        local.set 5
      end
      local.get 4
      i32.const 31
      i32.and
      local.set 1
      block  ;; label = @2
        local.get 4
        i32.const 223
        i32.gt_u
        br_if 0 (;@2;)
        local.get 5
        local.get 1
        i32.const 6
        i32.shl
        i32.or
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 2
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          local.set 3
          local.get 2
          local.set 6
          br 1 (;@2;)
        end
        local.get 0
        local.get 3
        i32.const 1
        i32.add
        local.tee 6
        i32.store
        local.get 3
        i32.load8_u
        i32.const 63
        i32.and
        local.set 3
      end
      local.get 3
      local.get 5
      i32.const 6
      i32.shl
      i32.or
      local.set 3
      block  ;; label = @2
        local.get 4
        i32.const 240
        i32.ge_u
        br_if 0 (;@2;)
        local.get 3
        local.get 1
        i32.const 12
        i32.shl
        i32.or
        return
      end
      block  ;; label = @2
        block  ;; label = @3
          local.get 6
          local.get 2
          i32.ne
          br_if 0 (;@3;)
          i32.const 0
          local.set 0
          br 1 (;@2;)
        end
        local.get 0
        local.get 6
        i32.const 1
        i32.add
        i32.store
        local.get 6
        i32.load8_u
        i32.const 63
        i32.and
        local.set 0
      end
      local.get 3
      i32.const 6
      i32.shl
      local.get 1
      i32.const 18
      i32.shl
      i32.const 1835008
      i32.and
      i32.or
      local.get 0
      i32.or
      local.set 4
    end
    local.get 4)
  (func $_ZN4core5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$15copy_from_slice17len_mismatch_fail17hca07929dff42a255E (type 8)
    unreachable
    unreachable)
  (func $_ZN4core3fmt8getcount17h1818d9452d192dbcE (type 2) (param i32 i32 i32)
    (local i32 i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 2
            i32.load
            br_table 0 (;@4;) 1 (;@3;) 3 (;@1;) 0 (;@4;)
          end
          local.get 2
          i32.const 4
          i32.add
          local.set 4
          br 1 (;@2;)
        end
        local.get 1
        local.get 2
        i32.load offset=4
        i32.const 3
        i32.shl
        i32.add
        local.tee 2
        i32.load offset=4
        i32.const 3
        i32.ne
        br_if 1 (;@1;)
        local.get 2
        i32.load
        local.set 4
      end
      local.get 4
      i32.load
      local.set 4
      i32.const 1
      local.set 3
    end
    local.get 0
    local.get 4
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store)
  (func $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h2d434b5b7cb25cb0E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 32
    i32.add
    i32.const 24
    i32.add
    local.set 4
    local.get 0
    i32.load offset=4
    local.set 5
    local.get 0
    i32.load
    local.set 6
    local.get 0
    i32.load offset=8
    local.set 7
    i32.const 0
    local.set 8
    block  ;; label = @1
      loop  ;; label = @2
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        block  ;; label = @3
          block  ;; label = @4
            local.get 7
            i32.load8_u
            i32.eqz
            br_if 0 (;@4;)
            local.get 6
            i32.const 65784
            i32.const 4
            local.get 5
            i32.load offset=12
            call_indirect (type 0)
            br_if 1 (;@3;)
          end
          local.get 3
          i32.const 10
          i32.store offset=56
          local.get 3
          i64.const 4294967306
          i64.store offset=48
          local.get 3
          local.get 2
          i32.store offset=44
          i32.const 0
          local.set 0
          local.get 3
          i32.const 0
          i32.store offset=40
          local.get 3
          local.get 2
          i32.store offset=36
          local.get 3
          local.get 1
          i32.store offset=32
          local.get 2
          local.set 9
          local.get 2
          local.set 10
          local.get 1
          local.set 11
          loop  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 9
                    local.get 0
                    i32.lt_u
                    local.get 9
                    local.get 10
                    i32.gt_u
                    i32.or
                    br_if 0 (;@8;)
                    block  ;; label = @9
                      local.get 3
                      i32.load offset=52
                      local.tee 10
                      i32.const -1
                      i32.add
                      local.tee 12
                      local.get 10
                      i32.gt_u
                      br_if 0 (;@9;)
                      local.get 11
                      local.get 0
                      i32.add
                      local.set 10
                      local.get 3
                      i32.const 32
                      i32.add
                      local.get 12
                      i32.add
                      i32.const 24
                      i32.add
                      i32.load8_u
                      local.set 11
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 9
                            local.get 0
                            i32.sub
                            local.tee 12
                            i32.const 8
                            i32.lt_u
                            br_if 0 (;@12;)
                            block  ;; label = @13
                              local.get 10
                              i32.const 3
                              i32.add
                              i32.const -4
                              i32.and
                              local.get 10
                              i32.sub
                              local.tee 9
                              br_if 0 (;@13;)
                              i32.const 0
                              local.set 9
                              br 2 (;@11;)
                            end
                            i32.const 0
                            local.set 0
                            local.get 3
                            i32.const 24
                            i32.add
                            i32.const 0
                            local.get 12
                            local.get 9
                            local.get 9
                            local.get 12
                            i32.gt_u
                            select
                            local.tee 9
                            local.get 10
                            local.get 12
                            call $_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h8df044a0ee528d6bE
                            local.get 3
                            i32.load offset=28
                            local.set 13
                            local.get 3
                            i32.load offset=24
                            local.set 14
                            loop  ;; label = @13
                              local.get 13
                              local.get 0
                              i32.eq
                              br_if 2 (;@11;)
                              local.get 14
                              local.get 0
                              i32.add
                              i32.load8_u
                              local.get 11
                              i32.eq
                              br_if 3 (;@10;)
                              local.get 0
                              i32.const 1
                              i32.add
                              local.tee 15
                              local.get 0
                              i32.lt_u
                              br_if 4 (;@9;)
                              local.get 15
                              local.set 0
                              br 0 (;@13;)
                            end
                          end
                          i32.const 0
                          local.set 0
                          loop  ;; label = @12
                            local.get 12
                            local.get 0
                            i32.eq
                            br_if 4 (;@8;)
                            local.get 10
                            local.get 0
                            i32.add
                            i32.load8_u
                            local.get 11
                            i32.eq
                            br_if 2 (;@10;)
                            local.get 0
                            i32.const 1
                            i32.add
                            local.set 0
                            br 0 (;@12;)
                          end
                        end
                        local.get 12
                        i32.const -8
                        i32.add
                        local.tee 14
                        local.get 12
                        i32.gt_u
                        br_if 1 (;@9;)
                        local.get 11
                        i32.const 16843009
                        i32.mul
                        local.set 0
                        block  ;; label = @11
                          loop  ;; label = @12
                            local.get 9
                            local.get 14
                            i32.gt_u
                            br_if 1 (;@11;)
                            local.get 9
                            i32.const 4
                            i32.add
                            local.tee 15
                            local.get 9
                            i32.lt_u
                            br_if 3 (;@9;)
                            local.get 10
                            local.get 9
                            i32.add
                            i32.load
                            local.get 0
                            i32.xor
                            local.tee 13
                            i32.const -1
                            i32.xor
                            local.get 13
                            i32.const -16843009
                            i32.add
                            i32.and
                            local.get 10
                            local.get 15
                            i32.add
                            i32.load
                            local.get 0
                            i32.xor
                            local.tee 15
                            i32.const -1
                            i32.xor
                            local.get 15
                            i32.const -16843009
                            i32.add
                            i32.and
                            i32.or
                            i32.const -2139062144
                            i32.and
                            br_if 1 (;@11;)
                            local.get 9
                            i32.const 8
                            i32.add
                            local.tee 15
                            local.get 9
                            i32.lt_u
                            br_if 3 (;@9;)
                            local.get 15
                            local.set 9
                            br 0 (;@12;)
                          end
                        end
                        local.get 12
                        local.get 9
                        i32.lt_u
                        br_if 4 (;@6;)
                        local.get 9
                        local.get 12
                        i32.sub
                        local.set 15
                        local.get 10
                        local.get 9
                        i32.add
                        local.set 10
                        i32.const 0
                        local.set 0
                        loop  ;; label = @11
                          local.get 15
                          local.get 0
                          i32.add
                          i32.eqz
                          br_if 3 (;@8;)
                          block  ;; label = @12
                            local.get 10
                            i32.load8_u
                            local.get 11
                            i32.eq
                            br_if 0 (;@12;)
                            local.get 0
                            i32.const 1
                            i32.add
                            local.tee 12
                            local.get 0
                            i32.lt_u
                            br_if 3 (;@9;)
                            local.get 10
                            i32.const 1
                            i32.add
                            local.set 10
                            local.get 12
                            local.set 0
                            br 1 (;@11;)
                          end
                        end
                        local.get 9
                        local.get 0
                        i32.add
                        local.tee 0
                        local.get 9
                        i32.lt_u
                        br_if 1 (;@9;)
                      end
                      local.get 0
                      i32.const 1
                      i32.add
                      local.tee 9
                      local.get 0
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 3
                      i32.load offset=40
                      local.tee 10
                      local.get 9
                      i32.add
                      local.tee 0
                      local.get 10
                      i32.lt_u
                      br_if 0 (;@9;)
                      local.get 3
                      local.get 0
                      i32.store offset=40
                      local.get 0
                      local.get 3
                      i32.load offset=52
                      local.tee 9
                      i32.lt_u
                      br_if 4 (;@5;)
                      local.get 0
                      local.get 3
                      i32.load offset=36
                      i32.gt_u
                      br_if 4 (;@5;)
                      local.get 3
                      i32.load offset=32
                      local.set 10
                      local.get 3
                      i32.const 16
                      i32.add
                      i32.const 0
                      local.get 9
                      local.get 4
                      i32.const 4
                      call $_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h8df044a0ee528d6bE
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 9
                          local.get 3
                          i32.load offset=20
                          i32.ne
                          br_if 0 (;@11;)
                          local.get 10
                          local.get 0
                          local.get 9
                          i32.sub
                          local.tee 11
                          i32.add
                          local.get 3
                          i32.load offset=16
                          local.get 9
                          call $memcmp
                          i32.eqz
                          br_if 1 (;@10;)
                        end
                        local.get 3
                        i32.load offset=40
                        local.set 0
                        br 5 (;@5;)
                      end
                      local.get 7
                      i32.const 1
                      i32.store8
                      local.get 11
                      i32.const 1
                      i32.add
                      local.tee 0
                      local.get 11
                      i32.ge_u
                      br_if 2 (;@7;)
                    end
                    unreachable
                    unreachable
                  end
                  local.get 7
                  i32.const 0
                  i32.store8
                  local.get 2
                  local.set 0
                end
                local.get 3
                i32.const 8
                i32.add
                local.get 0
                local.get 1
                local.get 2
                call $_ZN4core3str6traits110_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeTo$LT$usize$GT$$GT$5index17h21ffeabed6682ac4E
                local.get 6
                local.get 3
                i32.load offset=8
                local.get 3
                i32.load offset=12
                local.get 5
                i32.load offset=12
                call_indirect (type 0)
                br_if 3 (;@3;)
                local.get 3
                local.get 0
                local.get 1
                local.get 2
                call $_ZN4core3str6traits112_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeFrom$LT$usize$GT$$GT$5index17h246f1529e4a7a1f0E
                local.get 3
                i32.load offset=4
                local.set 2
                local.get 3
                i32.load
                local.set 1
                br 4 (;@2;)
              end
              call $_ZN4core5slice5index26slice_start_index_len_fail17h71cc3842941338daE
              unreachable
            end
            local.get 3
            i32.load offset=44
            local.set 9
            local.get 3
            i32.load offset=36
            local.set 10
            local.get 3
            i32.load offset=32
            local.set 11
            br 0 (;@4;)
          end
        end
      end
      i32.const 1
      local.set 8
    end
    local.get 3
    i32.const 64
    i32.add
    global.set 0
    local.get 8)
  (func $_ZN4core3str6traits110_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeTo$LT$usize$GT$$GT$5index17h21ffeabed6682ac4E (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    i32.const 8
    i32.add
    local.get 1
    local.get 2
    local.get 3
    call $_ZN4core3str6traits110_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeTo$LT$usize$GT$$GT$3get17he2f9dbd78df77a2eE
    block  ;; label = @1
      local.get 4
      i32.load offset=8
      local.tee 5
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      i32.const 0
      local.get 1
      call $_ZN4core3str16slice_error_fail17h2d866f60c2c4dd12E
      unreachable
    end
    local.get 4
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 5
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN4core3str6traits112_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeFrom$LT$usize$GT$$GT$5index17h246f1529e4a7a1f0E (type 13) (param i32 i32 i32 i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.eqz
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 1
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
          local.get 3
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.add
        i32.load8_s
        i32.const -64
        i32.lt_s
        br_if 1 (;@1;)
      end
      block  ;; label = @2
        local.get 3
        local.get 1
        i32.sub
        local.tee 4
        local.get 3
        i32.le_u
        br_if 0 (;@2;)
        unreachable
        unreachable
      end
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      local.get 2
      local.get 1
      i32.add
      i32.store
      return
    end
    local.get 2
    local.get 3
    local.get 1
    local.get 3
    call $_ZN4core3str16slice_error_fail17h2d866f60c2c4dd12E
    unreachable)
  (func $_ZN4core3str16slice_error_fail17h2d866f60c2c4dd12E (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    block  ;; label = @1
      local.get 1
      i32.const 257
      i32.lt_u
      br_if 0 (;@1;)
      i32.const 256
      local.set 5
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            local.get 5
            br_if 0 (;@4;)
            i32.const 0
            local.set 5
            br 2 (;@2;)
          end
          local.get 0
          local.get 5
          i32.add
          i32.load8_s
          i32.const -65
          i32.gt_s
          br_if 1 (;@2;)
          local.get 5
          i32.const -1
          i32.add
          local.set 5
          br 0 (;@3;)
        end
      end
      local.get 4
      i32.const 16
      i32.add
      local.get 5
      local.get 0
      local.get 1
      call $_ZN4core3str6traits110_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeTo$LT$usize$GT$$GT$5index17h21ffeabed6682ac4E
    end
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      local.get 3
      local.get 1
      i32.gt_u
      br_if 0 (;@1;)
      local.get 2
      local.get 3
      i32.gt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.eqz
          br_if 0 (;@3;)
          block  ;; label = @4
            local.get 2
            local.get 1
            i32.lt_u
            br_if 0 (;@4;)
            local.get 2
            local.get 1
            i32.eq
            br_if 1 (;@3;)
            br 2 (;@2;)
          end
          local.get 0
          local.get 2
          i32.add
          i32.load8_s
          i32.const -64
          i32.lt_s
          br_if 1 (;@2;)
        end
        local.get 3
        local.set 2
      end
      loop  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 2
              br_if 0 (;@5;)
              i32.const 0
              local.set 2
              br 1 (;@4;)
            end
            block  ;; label = @5
              local.get 2
              local.get 1
              i32.lt_u
              br_if 0 (;@5;)
              local.get 1
              local.get 2
              i32.ne
              br_if 2 (;@3;)
              local.get 1
              local.set 2
              br 1 (;@4;)
            end
            local.get 0
            local.get 2
            i32.add
            i32.load8_s
            i32.const -64
            i32.lt_s
            br_if 1 (;@3;)
          end
          local.get 4
          i32.const 8
          i32.add
          local.get 2
          local.get 0
          local.get 1
          call $_ZN4core3str6traits112_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeFrom$LT$usize$GT$$GT$5index17h246f1529e4a7a1f0E
          local.get 4
          local.get 4
          i32.load offset=8
          local.tee 5
          i32.store offset=24
          local.get 4
          local.get 5
          local.get 4
          i32.load offset=12
          i32.add
          i32.store offset=28
          local.get 4
          i32.const 24
          i32.add
          call $_ZN81_$LT$core..str..iter..Chars$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h36ebcf67f1b3ee3fE.34
          i32.const 1114112
          i32.ne
          drop
          br 2 (;@1;)
        end
        local.get 2
        i32.const -1
        i32.add
        local.set 2
        br 0 (;@2;)
      end
    end
    unreachable
    unreachable)
  (func $_ZN4core3ptr52drop_in_place$LT$core..fmt..builders..PadAdapter$GT$17h0124eecc74b0e386E (type 5) (param i32))
  (func $_ZN4core3fmt5Write10write_char17h20fafbc9c0c3193aE (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 128
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            i32.const 2048
            i32.lt_u
            br_if 1 (;@3;)
            local.get 1
            i32.const 65536
            i32.ge_u
            br_if 2 (;@2;)
            local.get 2
            local.get 1
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=14
            local.get 2
            local.get 1
            i32.const 12
            i32.shr_u
            i32.const 224
            i32.or
            i32.store8 offset=12
            local.get 2
            local.get 1
            i32.const 6
            i32.shr_u
            i32.const 63
            i32.and
            i32.const 128
            i32.or
            i32.store8 offset=13
            i32.const 3
            local.set 1
            br 3 (;@1;)
          end
          local.get 2
          local.get 1
          i32.store8 offset=12
          i32.const 1
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 1
        i32.const 63
        i32.and
        i32.const 128
        i32.or
        i32.store8 offset=13
        local.get 2
        local.get 1
        i32.const 6
        i32.shr_u
        i32.const 192
        i32.or
        i32.store8 offset=12
        i32.const 2
        local.set 1
        br 1 (;@1;)
      end
      local.get 2
      local.get 1
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=15
      local.get 2
      local.get 1
      i32.const 18
      i32.shr_u
      i32.const 240
      i32.or
      i32.store8 offset=12
      local.get 2
      local.get 1
      i32.const 6
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=14
      local.get 2
      local.get 1
      i32.const 12
      i32.shr_u
      i32.const 63
      i32.and
      i32.const 128
      i32.or
      i32.store8 offset=13
      i32.const 4
      local.set 1
    end
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    local.get 1
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h2d434b5b7cb25cb0E
    local.set 1
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3fmt5Write9write_fmt17h3247292a22916bc3E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=4
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 2
    i32.const 4
    i32.add
    i32.const 65800
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5write17ha77d2333e4c6ece8E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core3ptr64drop_in_place$LT$$RF$mut$u20$core..fmt..builders..PadAdapter$GT$17h2e2e01f8b4da3bd1E (type 5) (param i32))
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h7dd5d6dba37b9558E (type 0) (param i32 i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    local.get 2
    call $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h2d434b5b7cb25cb0E)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h1077bc9367c82bfeE (type 1) (param i32 i32) (result i32)
    local.get 0
    i32.load
    local.get 1
    call $_ZN4core3fmt5Write10write_char17h20fafbc9c0c3193aE)
  (func $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h5366af33edc29456E (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    local.get 1
    i64.load align=4
    i64.store offset=8
    local.get 0
    local.get 2
    i32.const 8
    i32.add
    call $_ZN4core3fmt5Write9write_fmt17h3247292a22916bc3E
    local.set 1
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 1)
  (func $_ZN4core7unicode9printable5check17h5b1565bc6091c57eE (type 17) (param i32 i32 i32 i32 i32 i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 7
    global.set 0
    local.get 1
    local.get 2
    i32.const 1
    i32.shl
    i32.add
    local.set 8
    local.get 0
    i32.const 65280
    i32.and
    i32.const 8
    i32.shr_u
    local.set 9
    i32.const 0
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        loop  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              local.get 1
              local.get 8
              i32.eq
              br_if 0 (;@5;)
              local.get 2
              local.get 1
              i32.load8_u offset=1
              i32.add
              local.tee 10
              local.get 2
              i32.lt_u
              br_if 4 (;@1;)
              local.get 1
              i32.const 2
              i32.add
              local.set 11
              local.get 1
              i32.load8_u
              i32.const 255
              i32.and
              local.tee 12
              local.get 9
              i32.eq
              br_if 1 (;@4;)
              local.get 11
              local.set 1
              local.get 10
              local.set 2
              local.get 12
              local.get 9
              i32.le_u
              br_if 2 (;@3;)
            end
            local.get 5
            local.get 6
            i32.add
            local.set 10
            local.get 0
            i32.const 65535
            i32.and
            local.set 2
            i32.const 1
            local.set 12
            loop  ;; label = @5
              local.get 5
              local.get 10
              i32.eq
              br_if 3 (;@2;)
              local.get 5
              i32.const 1
              i32.add
              local.set 0
              block  ;; label = @6
                local.get 5
                i32.load8_u
                local.tee 1
                i32.const 24
                i32.shl
                i32.const 24
                i32.shr_s
                local.tee 9
                i32.const 0
                i32.ge_s
                br_if 0 (;@6;)
                local.get 0
                local.get 10
                i32.eq
                br_if 5 (;@1;)
                local.get 5
                i32.const 2
                i32.add
                local.set 0
                local.get 9
                i32.const 127
                i32.and
                i32.const 8
                i32.shl
                local.get 5
                i32.load8_u offset=1
                i32.or
                local.set 1
              end
              local.get 1
              i32.const 0
              i32.gt_s
              local.get 2
              local.get 1
              i32.sub
              local.tee 1
              local.get 2
              i32.lt_s
              i32.xor
              br_if 4 (;@1;)
              local.get 1
              i32.const 0
              i32.lt_s
              br_if 3 (;@2;)
              local.get 12
              i32.const 1
              i32.xor
              local.set 12
              local.get 0
              local.set 5
              local.get 1
              local.set 2
              br 0 (;@5;)
            end
          end
          local.get 7
          i32.const 8
          i32.add
          local.get 2
          local.get 10
          local.get 3
          local.get 4
          call $_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h8df044a0ee528d6bE
          local.get 7
          i32.load offset=8
          local.set 2
          local.get 7
          i32.load offset=12
          local.set 1
          loop  ;; label = @4
            block  ;; label = @5
              local.get 1
              br_if 0 (;@5;)
              local.get 11
              local.set 1
              local.get 10
              local.set 2
              br 2 (;@3;)
            end
            local.get 1
            i32.const -1
            i32.add
            local.set 1
            local.get 2
            i32.load8_u
            local.set 12
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            local.get 12
            local.get 0
            i32.const 255
            i32.and
            i32.ne
            br_if 0 (;@4;)
          end
        end
        i32.const 0
        local.set 12
      end
      local.get 7
      i32.const 16
      i32.add
      global.set 0
      local.get 12
      i32.const 1
      i32.and
      return
    end
    unreachable
    unreachable)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17haf075c85251a1022E (type 1) (param i32 i32) (result i32)
    local.get 1
    i32.const 65824
    i32.const 2
    call $_ZN4core3fmt9Formatter3pad17h9ed4c84fd538e72aE)
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h14efe555c0bd4671E (type 1) (param i32 i32) (result i32)
    local.get 1
    i32.load offset=24
    i32.const 68432
    i32.const 5
    local.get 1
    i32.const 28
    i32.add
    i32.load
    i32.load offset=12
    call_indirect (type 0))
  (func $_ZN106_$LT$core..ops..range..Range$LT$usize$GT$$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$9index_mut17h6d040d6921c9ef33E (type 16) (param i32 i32 i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.gt_u
        br_if 0 (;@2;)
        block  ;; label = @3
          local.get 2
          local.get 4
          i32.gt_u
          br_if 0 (;@3;)
          local.get 2
          local.get 1
          i32.sub
          local.tee 4
          local.get 2
          i32.gt_u
          br_if 2 (;@1;)
          local.get 0
          local.get 4
          i32.store offset=4
          local.get 0
          local.get 3
          local.get 1
          i32.add
          i32.store
          return
        end
        call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
        unreachable
      end
      call $_ZN4core5slice5index22slice_index_order_fail17h1824dbaa9030e48aE
      unreachable
    end
    unreachable
    unreachable)
  (func $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$9push_byte17hcf753f1cf9d2ac5bE (type 4) (param i32 i32)
    (local i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=8
      local.tee 2
      local.get 0
      i32.load offset=4
      i32.ge_u
      br_if 0 (;@1;)
      local.get 0
      local.get 2
      i32.const 1
      i32.add
      i32.store offset=8
      local.get 0
      i32.load
      local.get 2
      i32.add
      local.get 1
      i32.store8
      return
    end
    unreachable
    unreachable)
  (func $_ZN4core3ptr57drop_in_place$LT$$RF$parity_scale_codec..error..Error$GT$17h5841659b91333c2fE (type 5) (param i32))
  (func $_ZN18parity_scale_codec5codec6Encode9encode_to17h2c4585371f7e3286E (type 4) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store offset=12
    local.get 1
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5alloc11collections5btree4node12slice_insert17h181267581c88791bE (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.tee 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.sub
        local.tee 5
        local.get 1
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const -1
        i32.add
        local.tee 1
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 0
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 1
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 0
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      local.get 3
      i32.store
      return
    end
    unreachable
    unreachable)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17ha548bed9e797b95bE (type 7) (param i32) (result i32)
    (local i64 i64 i64)
    local.get 0
    i64.load offset=32
    local.set 1
    local.get 0
    i64.const 1
    i64.store offset=32
    local.get 0
    local.get 1
    local.get 0
    i64.load
    local.tee 2
    i64.add
    local.tee 1
    i64.store
    local.get 0
    local.get 0
    i64.load offset=8
    local.tee 3
    local.get 1
    local.get 2
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 1
    i64.store offset=8
    local.get 0
    local.get 0
    i64.load offset=16
    local.tee 2
    local.get 1
    local.get 3
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 1
    i64.store offset=16
    local.get 0
    local.get 0
    i64.load offset=24
    local.get 1
    local.get 2
    i64.lt_u
    i64.extend_i32_u
    i64.add
    i64.store offset=24
    local.get 0)
  (func $_ZN69_$LT$$RF$$u5b$u8$u5d$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17ha43a819276410a89E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load offset=4
      local.tee 3
      local.get 2
      i32.lt_u
      local.tee 4
      br_if 0 (;@1;)
      local.get 1
      local.get 0
      i32.load
      local.tee 5
      local.get 2
      call $memcpy
      drop
      local.get 0
      local.get 3
      local.get 2
      i32.sub
      i32.store offset=4
      local.get 0
      local.get 5
      local.get 2
      i32.add
      i32.store
    end
    local.get 4)
  (func $_ZN102_$LT$parity_scale_codec..compact..PrefixInput$LT$T$GT$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17hc0140882c03a8a52E (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    local.get 0
    i32.load16_u offset=4
    local.set 3
    local.get 0
    i32.const 0
    i32.store8 offset=4
    block  ;; label = @1
      local.get 3
      i32.const 1
      i32.and
      br_if 0 (;@1;)
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call $_ZN69_$LT$$RF$$u5b$u8$u5d$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17ha43a819276410a89E
      return
    end
    local.get 1
    local.get 3
    i32.const 8
    i32.shr_u
    i32.store8
    local.get 0
    i32.load
    local.get 1
    i32.const 1
    i32.add
    local.get 2
    i32.const -1
    i32.add
    call $_ZN69_$LT$$RF$$u5b$u8$u5d$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17ha43a819276410a89E)
  (func $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E (type 4) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    i32.const 0
    local.set 3
    local.get 2
    i32.const 0
    i32.store offset=12
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        local.get 2
        i32.const 12
        i32.add
        i32.const 4
        call $_ZN69_$LT$$RF$$u5b$u8$u5d$$u20$as$u20$parity_scale_codec..codec..Input$GT$4read17ha43a819276410a89E
        br_if 0 (;@2;)
        local.get 2
        i32.load offset=12
        local.set 1
        br 1 (;@1;)
      end
      i32.const 1
      local.set 3
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225 (type 18) (param i32 i64) (result i32)
    (local i64 i64)
    local.get 0
    i64.load offset=32
    local.set 2
    local.get 0
    local.get 1
    i64.store offset=32
    local.get 0
    local.get 2
    local.get 0
    i64.load
    local.tee 1
    i64.add
    local.tee 2
    i64.store
    local.get 0
    local.get 0
    i64.load offset=8
    local.tee 3
    local.get 2
    local.get 1
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 1
    i64.store offset=8
    local.get 0
    local.get 0
    i64.load offset=16
    local.tee 2
    local.get 1
    local.get 3
    i64.lt_u
    i64.extend_i32_u
    i64.add
    local.tee 1
    i64.store offset=16
    local.get 0
    local.get 0
    i64.load offset=24
    local.get 1
    local.get 2
    i64.lt_u
    i64.extend_i32_u
    i64.add
    i64.store offset=24
    local.get 0)
  (func $_ZN90_$LT$ink_primitives..key_ptr..KeyPtr$u20$as$u20$ink_storage..traits..keyptr..ExtKeyPtr$GT$8next_for17hfb57d7af5e666bbcE (type 7) (param i32) (result i32)
    local.get 0
    i64.const 1
    call $_ZN14ink_primitives7key_ptr6KeyPtr10advance_by17hb5e0fb7f264efbc9E.225)
  (func $_ZN11ink_storage11collections5stash1_108_$LT$impl$u20$parity_scale_codec..codec..Decode$u20$for$u20$ink_storage..collections..stash..VacantEntry$GT$6decode17heeaed37a09162c5eE (type 4) (param i32 i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    local.get 1
    call $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E
    i32.const 1
    local.set 3
    block  ;; label = @1
      local.get 2
      i32.load offset=8
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=12
      local.set 4
      local.get 2
      local.get 1
      call $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E
      local.get 2
      i32.load
      br_if 0 (;@1;)
      local.get 2
      i32.load offset=4
      local.set 1
      local.get 0
      local.get 4
      i32.store offset=4
      local.get 0
      i32.const 8
      i32.add
      local.get 1
      i32.store
      i32.const 0
      local.set 3
    end
    local.get 0
    local.get 3
    i32.store
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func $_ZN5alloc5alloc15exchange_malloc17hd971f8d5100d1a68E.240 (type 1) (param i32 i32) (result i32)
    block  ;; label = @1
      local.get 0
      local.get 1
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      i32.eqz
      br_if 0 (;@1;)
      local.get 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17hbbff3628aecd05a3E (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 96
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=94
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243 (type 1) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call $_ZN87_$LT$ink_allocator..bump..BumpAllocator$u20$as$u20$core..alloc..global..GlobalAlloc$GT$5alloc17h3b5b87aeed817ef7E)
  (func $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h60030e9adf15a33cE (type 2) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.lt_u
      br_if 0 (;@1;)
      call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h40a2b90a4374113bE (type 2) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.lt_u
      br_if 0 (;@1;)
      call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17he9bc6c8782f9e8ceE (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load offset=4
    local.tee 3
    i32.const 4
    i32.add
    local.get 3
    i32.load16_u offset=94
    i32.const 1
    i32.add
    local.tee 4
    local.get 0
    i32.load offset=8
    local.tee 0
    local.get 1
    call $_ZN5alloc11collections5btree4node12slice_insert17h181267581c88791bE
    local.get 3
    i32.const 48
    i32.add
    local.tee 1
    local.get 4
    local.get 0
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17h7d85e35484fb29baE
    local.get 3
    local.get 4
    i32.store16 offset=94
    local.get 1
    local.get 0
    i32.const 2
    i32.shl
    i32.add)
  (func $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h9c6c530ccfa8ad4cE (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 144
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=94
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17hbfba168d5d1170a0E (type 2) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i32.load16_u offset=94
    local.set 4
    local.get 3
    i32.const 0
    i32.store8 offset=24
    local.get 3
    local.get 4
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=16
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        local.get 3
        i32.const 16
        i32.add
        call $_ZN4core4iter5range110_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..RangeInclusive$LT$A$GT$$GT$4next17h260844521081c9dfE
        local.get 3
        i32.load offset=8
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=12
        local.set 4
        local.get 3
        local.get 1
        i32.store offset=36
        local.get 3
        local.get 2
        i32.store offset=32
        local.get 3
        local.get 4
        i32.store offset=40
        local.get 3
        i32.const 32
        i32.add
        call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17he4d991720d4aa946E
        br 0 (;@2;)
      end
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 3
    i32.const 48
    i32.add
    global.set 0)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17h45eb42203db36e8eE (type 13) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 0
    i32.load offset=4
    local.tee 5
    i32.const 4
    i32.add
    local.get 5
    i32.load16_u offset=94
    local.tee 6
    i32.const 1
    i32.add
    local.tee 7
    local.get 0
    i32.load offset=8
    local.tee 8
    local.get 1
    call $_ZN5alloc11collections5btree4node12slice_insert17h181267581c88791bE
    local.get 5
    i32.const 48
    i32.add
    local.get 7
    local.get 8
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17h7d85e35484fb29baE
    block  ;; label = @1
      local.get 8
      i32.const 1
      i32.add
      local.tee 2
      local.get 8
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 1
      i32.add
      local.tee 1
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        local.get 6
        i32.const 2
        i32.add
        local.tee 8
        i32.ge_u
        br_if 0 (;@2;)
        local.get 8
        local.get 2
        i32.sub
        local.tee 6
        local.get 8
        i32.gt_u
        br_if 1 (;@1;)
        local.get 6
        i32.const -1
        i32.add
        local.tee 9
        local.get 6
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const 96
        i32.add
        local.tee 6
        local.get 1
        i32.const 2
        i32.shl
        i32.add
        local.get 6
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 9
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 5
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      i32.const 96
      i32.add
      local.get 3
      i32.store
      local.get 5
      local.get 7
      i32.store16 offset=94
      local.get 8
      local.get 2
      local.get 8
      local.get 2
      i32.gt_u
      select
      local.set 8
      local.get 0
      i32.load
      local.set 0
      block  ;; label = @2
        loop  ;; label = @3
          local.get 8
          local.get 2
          i32.eq
          br_if 1 (;@2;)
          local.get 4
          local.get 5
          i32.store offset=4
          local.get 4
          local.get 0
          i32.store
          local.get 4
          local.get 2
          i32.store offset=8
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 4
          call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17he4d991720d4aa946E
          br 0 (;@3;)
        end
      end
      local.get 4
      i32.const 16
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17he4d991720d4aa946E (type 5) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.const 2
    i32.shl
    i32.add
    i32.const 96
    i32.add
    i32.load
    local.tee 2
    local.get 0
    i32.store16 offset=92
    local.get 2
    local.get 1
    i32.store)
  (func $_ZN4core4iter5range110_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..RangeInclusive$LT$A$GT$$GT$4next17h260844521081c9dfE (type 4) (param i32 i32)
    (local i32 i32 i32)
    i32.const 0
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        local.get 1
        i32.load8_u offset=8
        i32.eqz
        br_if 0 (;@2;)
        br 1 (;@1;)
      end
      local.get 1
      i32.load
      local.tee 3
      local.get 1
      i32.load offset=4
      local.tee 4
      i32.gt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 3
        local.get 4
        i32.lt_u
        br_if 0 (;@2;)
        i32.const 1
        local.set 2
        local.get 1
        i32.const 1
        i32.store8 offset=8
        br 1 (;@1;)
      end
      i32.const 1
      local.set 2
      local.get 1
      local.get 3
      i32.const 1
      i32.add
      i32.store
    end
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store)
  (func $_ZN5alloc11collections5btree4node12slice_insert17h7d85e35484fb29baE (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.tee 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.sub
        local.tee 5
        local.get 1
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const -1
        i32.add
        local.tee 1
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 0
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 1
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 0
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      local.get 3
      i32.store
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17h084a0905094048e2E (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 96
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=94
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17he4fcea40c4f0de45E (type 2) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.lt_u
      br_if 0 (;@1;)
      call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hbfcf1d7d9f7ad73eE (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load offset=4
    local.tee 3
    i32.const 4
    i32.add
    local.get 3
    i32.load16_u offset=94
    i32.const 1
    i32.add
    local.tee 4
    local.get 0
    i32.load offset=8
    local.tee 0
    local.get 1
    call $_ZN5alloc11collections5btree4node12slice_insert17h181267581c88791bE
    local.get 3
    i32.const 48
    i32.add
    local.tee 1
    local.get 4
    local.get 0
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17h221f31206eed77d4E
    local.get 3
    local.get 4
    i32.store16 offset=94
    local.get 1
    local.get 0
    i32.const 2
    i32.shl
    i32.add)
  (func $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h801bf85f2302c549E (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 144
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=94
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h360c1fbf92f55ce4E (type 2) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i32.load16_u offset=94
    local.set 4
    local.get 3
    i32.const 0
    i32.store8 offset=24
    local.get 3
    local.get 4
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=16
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        local.get 3
        i32.const 16
        i32.add
        call $_ZN4core4iter5range110_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..RangeInclusive$LT$A$GT$$GT$4next17h260844521081c9dfE
        local.get 3
        i32.load offset=8
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=12
        local.set 4
        local.get 3
        local.get 1
        i32.store offset=36
        local.get 3
        local.get 2
        i32.store offset=32
        local.get 3
        local.get 4
        i32.store offset=40
        local.get 3
        i32.const 32
        i32.add
        call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6d015cf587f65ed9E
        br 0 (;@2;)
      end
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 3
    i32.const 48
    i32.add
    global.set 0)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hd94233843f500156E (type 13) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 0
    i32.load offset=4
    local.tee 5
    i32.const 4
    i32.add
    local.get 5
    i32.load16_u offset=94
    local.tee 6
    i32.const 1
    i32.add
    local.tee 7
    local.get 0
    i32.load offset=8
    local.tee 8
    local.get 1
    call $_ZN5alloc11collections5btree4node12slice_insert17h181267581c88791bE
    local.get 5
    i32.const 48
    i32.add
    local.get 7
    local.get 8
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17h221f31206eed77d4E
    block  ;; label = @1
      local.get 8
      i32.const 1
      i32.add
      local.tee 2
      local.get 8
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 1
      i32.add
      local.tee 1
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 1
        local.get 6
        i32.const 2
        i32.add
        local.tee 8
        i32.ge_u
        br_if 0 (;@2;)
        local.get 8
        local.get 2
        i32.sub
        local.tee 6
        local.get 8
        i32.gt_u
        br_if 1 (;@1;)
        local.get 6
        i32.const -1
        i32.add
        local.tee 9
        local.get 6
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const 96
        i32.add
        local.tee 6
        local.get 1
        i32.const 2
        i32.shl
        i32.add
        local.get 6
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 9
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 5
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      i32.const 96
      i32.add
      local.get 3
      i32.store
      local.get 5
      local.get 7
      i32.store16 offset=94
      local.get 8
      local.get 2
      local.get 8
      local.get 2
      i32.gt_u
      select
      local.set 8
      local.get 0
      i32.load
      local.set 0
      block  ;; label = @2
        loop  ;; label = @3
          local.get 8
          local.get 2
          i32.eq
          br_if 1 (;@2;)
          local.get 4
          local.get 5
          i32.store offset=4
          local.get 4
          local.get 0
          i32.store
          local.get 4
          local.get 2
          i32.store offset=8
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 4
          call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6d015cf587f65ed9E
          br 0 (;@3;)
        end
      end
      local.get 4
      i32.const 16
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6d015cf587f65ed9E (type 5) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.const 2
    i32.shl
    i32.add
    i32.const 96
    i32.add
    i32.load
    local.tee 2
    local.get 0
    i32.store16 offset=92
    local.get 2
    local.get 1
    i32.store)
  (func $_ZN5alloc11collections5btree4node12slice_insert17h221f31206eed77d4E (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.tee 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.sub
        local.tee 5
        local.get 1
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const -1
        i32.add
        local.tee 1
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 0
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 1
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 0
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      local.get 3
      i32.store
      return
    end
    unreachable
    unreachable)
  (func $_ZN76_$LT$$u5b$T$u3b$$u20$N$u5d$$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h01fc34d3619bb365E (type 4) (param i32 i32)
    local.get 1
    local.get 0
    i32.const 11
    call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E)
  (func $_ZN55_$LT$X$u20$as$u20$parity_scale_codec..codec..Encode$GT$9encode_to17h50e64f9885339a4cE (type 4) (param i32 i32)
    local.get 1
    local.get 0
    i32.load
    i32.const 32
    call $_ZN100_$LT$ink_env..engine..on_chain..buffer..EncodeScope$u20$as$u20$parity_scale_codec..codec..Output$GT$5write17h760e17eecb468b35E)
  (func $_ZN11ink_storage6traits7optspec20pull_packed_root_opt17ha3f29b1c536f4bc0E (type 4) (param i32 i32)
    (local i32 i64 i64)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 16384
    i32.store offset=36
    local.get 2
    i32.const 68528
    i32.store offset=32
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            local.get 2
            i32.const 32
            i32.add
            call $_ZN7ink_env6engine8on_chain3ext11get_storage17h79d15de933cf47bdE
            local.tee 1
            i32.const 3
            i32.eq
            br_if 0 (;@4;)
            local.get 1
            i32.const 13
            i32.ne
            br_if 3 (;@1;)
            local.get 2
            local.get 2
            i64.load offset=32
            i64.store offset=40
            local.get 2
            i32.const 8
            i32.add
            local.get 2
            i32.const 40
            i32.add
            call $_ZN58_$LT$u128$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h874838e534925649E
            block  ;; label = @5
              local.get 2
              i32.load offset=8
              br_if 0 (;@5;)
              local.get 2
              i32.const 24
              i32.add
              i64.load
              local.set 3
              local.get 2
              i64.load offset=16
              local.set 4
              local.get 2
              local.get 2
              i32.const 40
              i32.add
              call $_ZN57_$LT$u32$u20$as$u20$parity_scale_codec..codec..Decode$GT$6decode17h4477965a6fa6ab30E
              local.get 2
              i32.load
              i32.eqz
              br_if 2 (;@3;)
            end
            call $_ZN4core6result13unwrap_failed17h2b5eb3392bf9d869E
            unreachable
          end
          i64.const 0
          local.set 3
          br 1 (;@2;)
        end
        local.get 2
        i32.load offset=4
        local.set 1
        local.get 0
        i32.const 16
        i32.add
        local.get 3
        i64.store
        local.get 0
        local.get 4
        i64.store offset=8
        local.get 0
        i32.const 24
        i32.add
        local.get 1
        i32.store
        i64.const 1
        local.set 3
      end
      local.get 0
      local.get 3
      i64.store
      local.get 2
      i32.const 48
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17h669c0a87dfa579baE (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 756
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=50
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN60_$LT$ink_env..types..AccountId$u20$as$u20$core..cmp..Ord$GT$3cmp17h3e35a495ccaba03eE (type 1) (param i32 i32) (result i32)
    i32.const -1
    i32.const 1
    local.get 0
    local.get 1
    i32.const 32
    call $memcmp
    local.tee 0
    i32.const 0
    i32.lt_s
    select
    i32.const 0
    local.get 0
    select)
  (func $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17h26193625bee7f760E (type 2) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.lt_u
      br_if 0 (;@1;)
      call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17hda973abfff99f4dfE (type 2) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.lt_u
      br_if 0 (;@1;)
      call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17he03aa9c1ee608749E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 3
    global.set 0
    local.get 0
    i32.load offset=4
    local.tee 4
    i32.const 52
    i32.add
    local.get 4
    i32.load16_u offset=50
    i32.const 1
    i32.add
    local.tee 5
    local.get 0
    i32.load offset=8
    local.tee 0
    local.get 3
    local.get 1
    i32.const 64
    call $memcpy
    local.tee 1
    call $_ZN5alloc11collections5btree4node12slice_insert17h111ee4d6a3cffc3bE
    local.get 4
    i32.const 4
    i32.add
    local.tee 3
    local.get 5
    local.get 0
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17hca13e3ecff01dbf7E
    local.get 4
    local.get 5
    i32.store16 offset=50
    local.get 1
    i32.const 64
    i32.add
    global.set 0
    local.get 3
    local.get 0
    i32.const 2
    i32.shl
    i32.add)
  (func $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17h19f31af566aba95bE (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 804
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=50
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h4750e4b371ff1a83E (type 2) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i32.load16_u offset=50
    local.set 4
    local.get 3
    i32.const 0
    i32.store8 offset=24
    local.get 3
    local.get 4
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=16
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        local.get 3
        i32.const 16
        i32.add
        call $_ZN4core4iter5range110_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..RangeInclusive$LT$A$GT$$GT$4next17h260844521081c9dfE
        local.get 3
        i32.load offset=8
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=12
        local.set 4
        local.get 3
        local.get 1
        i32.store offset=36
        local.get 3
        local.get 2
        i32.store offset=32
        local.get 3
        local.get 4
        i32.store offset=40
        local.get 3
        i32.const 32
        i32.add
        call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6179d7c09eaf4df4E
        br 0 (;@2;)
      end
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 3
    i32.const 48
    i32.add
    global.set 0)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17h3f7d3232dc6e2d8aE (type 13) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 64
    i32.sub
    local.tee 4
    global.set 0
    local.get 0
    i32.load offset=4
    local.tee 5
    i32.const 52
    i32.add
    local.get 5
    i32.load16_u offset=50
    local.tee 6
    i32.const 1
    i32.add
    local.tee 7
    local.get 0
    i32.load offset=8
    local.tee 8
    local.get 4
    local.get 1
    i32.const 64
    call $memcpy
    local.tee 1
    call $_ZN5alloc11collections5btree4node12slice_insert17h111ee4d6a3cffc3bE
    local.get 5
    i32.const 4
    i32.add
    local.get 7
    local.get 8
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17hca13e3ecff01dbf7E
    block  ;; label = @1
      local.get 8
      i32.const 1
      i32.add
      local.tee 2
      local.get 8
      i32.lt_u
      br_if 0 (;@1;)
      local.get 2
      i32.const 1
      i32.add
      local.tee 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        local.get 6
        i32.const 2
        i32.add
        local.tee 8
        i32.ge_u
        br_if 0 (;@2;)
        local.get 8
        local.get 2
        i32.sub
        local.tee 6
        local.get 8
        i32.gt_u
        br_if 1 (;@1;)
        local.get 6
        i32.const -1
        i32.add
        local.tee 9
        local.get 6
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const 756
        i32.add
        local.tee 6
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 6
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 9
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 5
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      i32.const 756
      i32.add
      local.get 3
      i32.store
      local.get 5
      local.get 7
      i32.store16 offset=50
      local.get 8
      local.get 2
      local.get 8
      local.get 2
      i32.gt_u
      select
      local.set 8
      local.get 0
      i32.load
      local.set 0
      block  ;; label = @2
        loop  ;; label = @3
          local.get 8
          local.get 2
          i32.eq
          br_if 1 (;@2;)
          local.get 1
          local.get 5
          i32.store offset=4
          local.get 1
          local.get 0
          i32.store
          local.get 1
          local.get 2
          i32.store offset=8
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          local.get 1
          call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6179d7c09eaf4df4E
          br 0 (;@3;)
        end
      end
      local.get 1
      i32.const 64
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17h6179d7c09eaf4df4E (type 5) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.const 2
    i32.shl
    i32.add
    i32.const 756
    i32.add
    i32.load
    local.tee 2
    local.get 0
    i32.store16 offset=48
    local.get 2
    local.get 1
    i32.store)
  (func $_ZN5alloc11collections5btree4node12slice_insert17h111ee4d6a3cffc3bE (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.tee 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.sub
        local.tee 5
        local.get 1
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const -1
        i32.add
        local.tee 1
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 4
        i32.const 6
        i32.shl
        i32.add
        local.get 0
        local.get 2
        i32.const 6
        i32.shl
        i32.add
        local.get 1
        i32.const 6
        i32.shl
        call $memmove
        drop
      end
      local.get 0
      local.get 2
      i32.const 6
      i32.shl
      i32.add
      local.get 3
      i32.const 64
      call $memcpy
      drop
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node12slice_insert17hca13e3ecff01dbf7E (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.tee 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.sub
        local.tee 5
        local.get 1
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const -1
        i32.add
        local.tee 1
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 4
        i32.const 2
        i32.shl
        i32.add
        local.get 0
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 1
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 0
      local.get 2
      i32.const 2
      i32.shl
      i32.add
      local.get 3
      i32.store
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node21LeafNode$LT$K$C$V$GT$3new17hf25fb16daee1c84eE (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 404
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=50
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN4core5array88_$LT$impl$u20$core..ops..index..IndexMut$LT$I$GT$$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$9index_mut17ha96656558948c07cE (type 2) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 12
      i32.lt_u
      br_if 0 (;@1;)
      call $_ZN4core5slice5index24slice_end_index_len_fail17ha85ae06de35adabeE
      unreachable
    end
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func $_ZN5alloc11collections5btree4node210Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Leaf$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hd472ed3c7a763cb1E (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 0
    i32.load offset=8
    local.set 4
    local.get 0
    i32.load offset=4
    local.tee 0
    i32.load16_u offset=50
    local.set 5
    local.get 3
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get 3
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get 3
    local.get 1
    i64.load align=1
    i64.store
    local.get 0
    i32.const 52
    i32.add
    local.get 5
    i32.const 1
    i32.add
    local.tee 1
    local.get 4
    local.get 3
    call $_ZN5alloc11collections5btree4node12slice_insert17h413277a7e3ddf588E
    local.get 0
    i32.const 4
    i32.add
    local.tee 5
    local.get 1
    local.get 4
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17hca13e3ecff01dbf7E
    local.get 0
    local.get 1
    i32.store16 offset=50
    local.get 3
    i32.const 32
    i32.add
    global.set 0
    local.get 5
    local.get 4
    i32.const 2
    i32.shl
    i32.add)
  (func $_ZN5alloc11collections5btree4node25InternalNode$LT$K$C$V$GT$3new17haec4b1edce4a03b0E (type 15) (result i32)
    (local i32)
    block  ;; label = @1
      i32.const 452
      i32.const 4
      call $_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$8allocate17h086126c6c4b5d84eE.243
      local.tee 0
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.const 0
    i32.store16 offset=50
    local.get 0
    i32.const 0
    i32.store
    local.get 0)
  (func $_ZN5alloc11collections5btree4node121NodeRef$LT$alloc..collections..btree..node..marker..Owned$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$17from_new_internal17h09b48f3afe627b6eE (type 2) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    local.get 1
    i32.load16_u offset=50
    local.set 4
    local.get 3
    i32.const 0
    i32.store8 offset=24
    local.get 3
    local.get 4
    i32.store offset=20
    local.get 3
    i32.const 0
    i32.store offset=16
    block  ;; label = @1
      loop  ;; label = @2
        local.get 3
        i32.const 8
        i32.add
        local.get 3
        i32.const 16
        i32.add
        call $_ZN4core4iter5range110_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..RangeInclusive$LT$A$GT$$GT$4next17h260844521081c9dfE
        local.get 3
        i32.load offset=8
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        i32.load offset=12
        local.set 4
        local.get 3
        local.get 1
        i32.store offset=36
        local.get 3
        local.get 2
        i32.store offset=32
        local.get 3
        local.get 4
        i32.store offset=40
        local.get 3
        i32.const 32
        i32.add
        call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17hee3182ae6ccf1120E
        br 0 (;@2;)
      end
    end
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 3
    i32.const 48
    i32.add
    global.set 0)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$10insert_fit17hde9b41d070f2143aE (type 13) (param i32 i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 0
    i32.load offset=8
    local.set 5
    local.get 0
    i32.load offset=4
    local.tee 6
    i32.load16_u offset=50
    local.set 7
    local.get 4
    i32.const 24
    i32.add
    local.get 1
    i32.const 24
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    i32.const 16
    i32.add
    local.get 1
    i32.const 16
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    i32.const 8
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i64.load align=1
    i64.store
    local.get 4
    local.get 1
    i64.load align=1
    i64.store
    local.get 6
    i32.const 52
    i32.add
    local.get 7
    i32.const 1
    i32.add
    local.tee 8
    local.get 5
    local.get 4
    call $_ZN5alloc11collections5btree4node12slice_insert17h413277a7e3ddf588E
    local.get 6
    i32.const 4
    i32.add
    local.get 8
    local.get 5
    local.get 2
    call $_ZN5alloc11collections5btree4node12slice_insert17hca13e3ecff01dbf7E
    block  ;; label = @1
      local.get 5
      i32.const 1
      i32.add
      local.tee 1
      local.get 5
      i32.lt_u
      br_if 0 (;@1;)
      local.get 1
      i32.const 1
      i32.add
      local.tee 2
      local.get 1
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 2
        local.get 7
        i32.const 2
        i32.add
        local.tee 5
        i32.ge_u
        br_if 0 (;@2;)
        local.get 5
        local.get 1
        i32.sub
        local.tee 7
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 7
        i32.const -1
        i32.add
        local.tee 9
        local.get 7
        i32.gt_u
        br_if 1 (;@1;)
        local.get 6
        i32.const 404
        i32.add
        local.tee 7
        local.get 2
        i32.const 2
        i32.shl
        i32.add
        local.get 7
        local.get 1
        i32.const 2
        i32.shl
        i32.add
        local.get 9
        i32.const 2
        i32.shl
        call $memmove
        drop
      end
      local.get 6
      local.get 1
      i32.const 2
      i32.shl
      i32.add
      i32.const 404
      i32.add
      local.get 3
      i32.store
      local.get 6
      local.get 8
      i32.store16 offset=50
      local.get 5
      local.get 1
      local.get 5
      local.get 1
      i32.gt_u
      select
      local.set 5
      local.get 0
      i32.load
      local.set 0
      block  ;; label = @2
        loop  ;; label = @3
          local.get 5
          local.get 1
          i32.eq
          br_if 1 (;@2;)
          local.get 4
          local.get 6
          i32.store offset=4
          local.get 4
          local.get 0
          i32.store
          local.get 4
          local.get 1
          i32.store offset=8
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 4
          call $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17hee3182ae6ccf1120E
          br 0 (;@3;)
        end
      end
      local.get 4
      i32.const 32
      i32.add
      global.set 0
      return
    end
    unreachable
    unreachable)
  (func $_ZN5alloc11collections5btree4node214Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Mut$C$K$C$V$C$alloc..collections..btree..node..marker..Internal$GT$$C$alloc..collections..btree..node..marker..Edge$GT$19correct_parent_link17hee3182ae6ccf1120E (type 5) (param i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 0
      i32.load
      br_if 0 (;@1;)
      unreachable
      unreachable
    end
    local.get 0
    i32.load offset=4
    local.tee 1
    local.get 0
    i32.load offset=8
    local.tee 0
    i32.const 2
    i32.shl
    i32.add
    i32.const 404
    i32.add
    i32.load
    local.tee 2
    local.get 0
    i32.store16 offset=48
    local.get 2
    local.get 1
    i32.store)
  (func $_ZN5alloc11collections5btree4node12slice_insert17h413277a7e3ddf588E (type 13) (param i32 i32 i32 i32)
    (local i32 i32)
    block  ;; label = @1
      local.get 2
      i32.const 1
      i32.add
      local.tee 4
      local.get 2
      i32.lt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        local.get 4
        local.get 1
        i32.ge_u
        br_if 0 (;@2;)
        local.get 1
        local.get 2
        i32.sub
        local.tee 5
        local.get 1
        i32.gt_u
        br_if 1 (;@1;)
        local.get 5
        i32.const -1
        i32.add
        local.tee 1
        local.get 5
        i32.gt_u
        br_if 1 (;@1;)
        local.get 0
        local.get 4
        i32.const 5
        i32.shl
        i32.add
        local.get 0
        local.get 2
        i32.const 5
        i32.shl
        i32.add
        local.get 1
        i32.const 5
        i32.shl
        call $memmove
        drop
      end
      local.get 0
      local.get 2
      i32.const 5
      i32.shl
      i32.add
      local.tee 2
      local.get 3
      i64.load align=1
      i64.store align=1
      local.get 2
      i32.const 24
      i32.add
      local.get 3
      i32.const 24
      i32.add
      i64.load align=1
      i64.store align=1
      local.get 2
      i32.const 16
      i32.add
      local.get 3
      i32.const 16
      i32.add
      i64.load align=1
      i64.store align=1
      local.get 2
      i32.const 8
      i32.add
      local.get 3
      i32.const 8
      i32.add
      i64.load align=1
      i64.store align=1
      return
    end
    unreachable
    unreachable)
  (func $_ZN4core3ptr46drop_in_place$LT$$RF$alloc..string..String$GT$17hbb500e04f37bfd59E (type 5) (param i32))
  (func $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf89e08cb0223db08E (type 1) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 0
    i32.load
    local.tee 0
    i32.load offset=8
    local.set 3
    local.get 0
    i32.load
    local.set 4
    i32.const 1
    local.set 0
    block  ;; label = @1
      local.get 1
      i32.load offset=24
      local.tee 5
      i32.const 34
      local.get 1
      i32.const 28
      i32.add
      i32.load
      local.tee 6
      i32.load offset=16
      local.tee 7
      call_indirect (type 1)
      br_if 0 (;@1;)
      local.get 2
      local.get 4
      i32.store offset=20
      local.get 2
      i32.const 24
      i32.add
      local.get 4
      local.get 3
      i32.add
      i32.store
      i32.const 0
      local.set 8
      local.get 2
      i32.const 0
      i32.store offset=16
      block  ;; label = @2
        block  ;; label = @3
          loop  ;; label = @4
            local.get 2
            i32.const 8
            i32.add
            local.get 2
            i32.const 16
            i32.add
            call $_ZN87_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hfb31d1bde6bbf7ffE
            i32.const 2
            local.set 1
            i32.const 116
            local.set 9
            local.get 2
            i32.load offset=8
            local.set 10
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 2
                          i32.load offset=12
                          local.tee 11
                          i32.const -9
                          i32.add
                          br_table 6 (;@5;) 2 (;@9;) 4 (;@7;) 4 (;@7;) 1 (;@10;) 0 (;@11;)
                        end
                        local.get 11
                        i32.const 34
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 11
                        i32.const 92
                        i32.eq
                        br_if 2 (;@8;)
                        local.get 11
                        i32.const 1114112
                        i32.ne
                        br_if 3 (;@7;)
                        local.get 2
                        local.get 8
                        local.get 4
                        local.get 3
                        call $_ZN4core3str6traits112_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..RangeFrom$LT$usize$GT$$GT$5index17h246f1529e4a7a1f0E
                        i32.const 1
                        local.set 0
                        local.get 5
                        local.get 2
                        i32.load
                        local.get 2
                        i32.load offset=4
                        local.get 6
                        i32.load offset=12
                        call_indirect (type 0)
                        br_if 9 (;@1;)
                        local.get 5
                        i32.const 34
                        local.get 7
                        call_indirect (type 1)
                        local.set 0
                        br 9 (;@1;)
                      end
                      i32.const 114
                      local.set 9
                      br 4 (;@5;)
                    end
                    i32.const 110
                    local.set 9
                    br 3 (;@5;)
                  end
                  br 1 (;@6;)
                end
                local.get 11
                i32.const 11
                i32.shl
                local.set 9
                i32.const 0
                local.set 1
                i32.const 31
                local.set 0
                i32.const 31
                local.set 12
                block  ;; label = @7
                  loop  ;; label = @8
                    local.get 1
                    local.get 0
                    i32.ge_u
                    br_if 1 (;@7;)
                    local.get 1
                    local.get 12
                    i32.const 1
                    i32.shr_u
                    i32.add
                    local.tee 12
                    local.get 1
                    i32.lt_u
                    br_if 6 (;@2;)
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          local.get 12
                          i32.const 2
                          i32.shl
                          i32.const 67436
                          i32.add
                          i32.load
                          i32.const 11
                          i32.shl
                          local.tee 13
                          local.get 9
                          i32.lt_u
                          br_if 0 (;@11;)
                          local.get 13
                          local.get 9
                          i32.eq
                          br_if 2 (;@9;)
                          local.get 12
                          local.set 0
                          br 1 (;@10;)
                        end
                        local.get 12
                        i32.const 1
                        i32.add
                        local.tee 1
                        local.get 12
                        i32.lt_u
                        br_if 8 (;@2;)
                      end
                      local.get 0
                      local.get 1
                      i32.sub
                      local.tee 12
                      local.get 0
                      i32.le_u
                      br_if 1 (;@8;)
                      br 7 (;@2;)
                    end
                  end
                  local.get 12
                  i32.const 1
                  i32.add
                  local.set 1
                end
                local.get 1
                i32.const 30
                i32.gt_u
                br_if 4 (;@2;)
                local.get 1
                i32.const 2
                i32.shl
                local.tee 12
                i32.const 67436
                i32.add
                i32.load
                i32.const 21
                i32.shr_u
                local.set 0
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.const 30
                    i32.eq
                    br_if 0 (;@8;)
                    local.get 12
                    i32.const 67440
                    i32.add
                    i32.load
                    i32.const 21
                    i32.shr_u
                    local.tee 9
                    local.get 0
                    i32.sub
                    local.tee 12
                    local.get 9
                    i32.gt_u
                    br_if 6 (;@2;)
                    br 1 (;@7;)
                  end
                  i32.const 689
                  local.get 0
                  i32.sub
                  local.tee 12
                  i32.const 690
                  i32.ge_u
                  br_if 5 (;@2;)
                end
                i32.const 0
                local.set 9
                block  ;; label = @7
                  local.get 1
                  i32.const -1
                  i32.add
                  local.tee 13
                  local.get 1
                  i32.gt_u
                  br_if 0 (;@7;)
                  local.get 13
                  i32.const 31
                  i32.ge_u
                  br_if 5 (;@2;)
                  local.get 13
                  i32.const 2
                  i32.shl
                  i32.const 67436
                  i32.add
                  i32.load
                  i32.const 2097151
                  i32.and
                  local.set 9
                end
                local.get 11
                local.get 9
                i32.sub
                local.tee 14
                local.get 11
                i32.gt_u
                br_if 4 (;@2;)
                local.get 12
                i32.const -1
                i32.add
                local.tee 1
                local.get 12
                i32.gt_u
                br_if 4 (;@2;)
                local.get 0
                i32.const 689
                local.get 0
                i32.const 689
                i32.gt_u
                select
                local.set 13
                local.get 0
                local.get 12
                i32.add
                i32.const -1
                i32.add
                local.set 15
                i32.const 0
                local.set 12
                loop  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      local.get 1
                      i32.eqz
                      br_if 0 (;@9;)
                      local.get 13
                      local.get 0
                      i32.eq
                      br_if 7 (;@2;)
                      local.get 12
                      local.get 0
                      i32.const 67560
                      i32.add
                      i32.load8_u
                      i32.add
                      local.tee 9
                      local.get 12
                      i32.lt_u
                      br_if 7 (;@2;)
                      local.get 9
                      local.get 14
                      i32.le_u
                      br_if 1 (;@8;)
                      local.get 0
                      local.set 15
                    end
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            local.get 15
                            i32.const 1
                            i32.and
                            br_if 0 (;@12;)
                            local.get 11
                            i32.const 65536
                            i32.lt_u
                            br_if 1 (;@11;)
                            block  ;; label = @13
                              local.get 11
                              i32.const 131072
                              i32.ge_u
                              br_if 0 (;@13;)
                              local.get 11
                              i32.const 66763
                              i32.const 38
                              i32.const 66839
                              i32.const 175
                              i32.const 67014
                              i32.const 419
                              call $_ZN4core7unicode9printable5check17h5b1565bc6091c57eE
                              br_if 9 (;@4;)
                              br 3 (;@10;)
                            end
                            local.get 11
                            i32.const -173790
                            i32.add
                            i32.const 34
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 11
                            i32.const -177973
                            i32.add
                            i32.const 11
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 11
                            i32.const 2097150
                            i32.and
                            i32.const 178206
                            i32.eq
                            br_if 2 (;@10;)
                            local.get 11
                            i32.const -183970
                            i32.add
                            i32.const 14
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 11
                            i32.const -191457
                            i32.add
                            i32.const 3103
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 11
                            i32.const -195102
                            i32.add
                            i32.const 1506
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 11
                            i32.const -201547
                            i32.add
                            i32.const 716213
                            i32.lt_u
                            br_if 2 (;@10;)
                            local.get 11
                            i32.const 918000
                            i32.lt_u
                            br_if 8 (;@4;)
                            br 2 (;@10;)
                          end
                          local.get 11
                          i32.const 1
                          i32.or
                          i32.clz
                          i32.const 2
                          i32.shr_u
                          i32.const 7
                          i32.xor
                          i64.extend_i32_u
                          i64.const 21474836480
                          i64.or
                          local.set 16
                          br 2 (;@9;)
                        end
                        local.get 11
                        i32.const 66082
                        i32.const 41
                        i32.const 66164
                        i32.const 290
                        i32.const 66454
                        i32.const 309
                        call $_ZN4core7unicode9printable5check17h5b1565bc6091c57eE
                        br_if 6 (;@4;)
                      end
                      local.get 11
                      i32.const 1
                      i32.or
                      i32.clz
                      i32.const 2
                      i32.shr_u
                      i32.const 7
                      i32.xor
                      i64.extend_i32_u
                      i64.const 21474836480
                      i64.or
                      local.set 16
                    end
                    local.get 16
                    i32.wrap_i64
                    local.tee 1
                    local.get 16
                    i64.const 32
                    i64.shr_u
                    i32.wrap_i64
                    i32.add
                    local.tee 0
                    local.get 1
                    i32.lt_u
                    br_if 6 (;@2;)
                    local.get 0
                    i32.const 1
                    i32.eq
                    br_if 4 (;@4;)
                    i32.const 3
                    local.set 1
                    br 2 (;@6;)
                  end
                  local.get 1
                  i32.const -1
                  i32.add
                  local.set 1
                  local.get 0
                  i32.const 1
                  i32.add
                  local.set 0
                  local.get 9
                  local.set 12
                  br 0 (;@7;)
                end
              end
              local.get 11
              local.set 9
            end
            local.get 8
            local.get 10
            i32.gt_u
            br_if 1 (;@3;)
            block  ;; label = @5
              local.get 8
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 8
                local.get 3
                i32.lt_u
                br_if 0 (;@6;)
                local.get 3
                local.get 8
                i32.eq
                br_if 1 (;@5;)
                br 3 (;@3;)
              end
              local.get 4
              local.get 8
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 2 (;@3;)
            end
            block  ;; label = @5
              local.get 10
              i32.eqz
              br_if 0 (;@5;)
              block  ;; label = @6
                local.get 10
                local.get 3
                i32.lt_u
                br_if 0 (;@6;)
                local.get 3
                local.get 10
                i32.ne
                br_if 3 (;@3;)
                br 1 (;@5;)
              end
              local.get 4
              local.get 10
              i32.add
              i32.load8_s
              i32.const -64
              i32.lt_s
              br_if 2 (;@3;)
            end
            local.get 10
            local.get 8
            i32.sub
            local.tee 0
            local.get 10
            i32.gt_u
            br_if 2 (;@2;)
            block  ;; label = @5
              local.get 5
              local.get 4
              local.get 8
              i32.add
              local.get 0
              local.get 6
              i32.load offset=12
              call_indirect (type 0)
              br_if 0 (;@5;)
              loop  ;; label = @6
                local.get 1
                local.set 12
                i32.const 0
                local.set 1
                local.get 9
                local.set 0
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        local.get 12
                        br_table 1 (;@9;) 3 (;@7;) 2 (;@8;) 0 (;@10;) 1 (;@9;)
                      end
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                local.get 16
                                i64.const 32
                                i64.shr_u
                                i32.wrap_i64
                                i32.const 255
                                i32.and
                                br_table 5 (;@9;) 0 (;@14;) 4 (;@10;) 1 (;@13;) 2 (;@12;) 3 (;@11;) 5 (;@9;)
                              end
                              local.get 16
                              i64.const -1095216660481
                              i64.and
                              local.set 16
                              i32.const 125
                              local.set 0
                              i32.const 3
                              local.set 1
                              br 6 (;@7;)
                            end
                            local.get 16
                            i64.const -1095216660481
                            i64.and
                            i64.const 8589934592
                            i64.or
                            local.set 16
                            i32.const 123
                            local.set 0
                            i32.const 3
                            local.set 1
                            br 5 (;@7;)
                          end
                          local.get 16
                          i64.const -1095216660481
                          i64.and
                          i64.const 12884901888
                          i64.or
                          local.set 16
                          i32.const 117
                          local.set 0
                          i32.const 3
                          local.set 1
                          br 4 (;@7;)
                        end
                        local.get 16
                        i64.const -1095216660481
                        i64.and
                        i64.const 17179869184
                        i64.or
                        local.set 16
                        i32.const 92
                        local.set 0
                        i32.const 3
                        local.set 1
                        br 3 (;@7;)
                      end
                      local.get 16
                      i32.wrap_i64
                      local.tee 1
                      i32.const 1073741823
                      i32.and
                      local.get 1
                      i32.ne
                      br_if 7 (;@2;)
                      local.get 1
                      i32.const 2
                      i32.shl
                      local.tee 0
                      i32.const 32
                      i32.ge_u
                      br_if 7 (;@2;)
                      local.get 9
                      local.get 0
                      i32.const 28
                      i32.and
                      i32.shr_u
                      i32.const 15
                      i32.and
                      local.tee 0
                      i32.const 48
                      i32.or
                      local.get 0
                      i32.const 87
                      i32.add
                      local.get 0
                      i32.const 10
                      i32.lt_u
                      select
                      local.set 0
                      block  ;; label = @10
                        local.get 1
                        br_if 0 (;@10;)
                        local.get 16
                        i64.const -1095216660481
                        i64.and
                        i64.const 4294967296
                        i64.or
                        local.set 16
                        i32.const 3
                        local.set 1
                        br 3 (;@7;)
                      end
                      local.get 16
                      i64.const -4294967296
                      i64.and
                      local.get 16
                      i64.const -1
                      i64.add
                      i64.const 4294967295
                      i64.and
                      i64.or
                      local.set 16
                      i32.const 3
                      local.set 1
                      br 2 (;@7;)
                    end
                    i32.const 1
                    local.set 1
                    block  ;; label = @9
                      local.get 11
                      i32.const 128
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 2
                      local.set 1
                      local.get 11
                      i32.const 2048
                      i32.lt_u
                      br_if 0 (;@9;)
                      i32.const 3
                      i32.const 4
                      local.get 11
                      i32.const 65536
                      i32.lt_u
                      select
                      local.set 1
                    end
                    local.get 10
                    local.get 1
                    i32.add
                    local.tee 8
                    local.get 10
                    i32.ge_u
                    br_if 4 (;@4;)
                    br 6 (;@2;)
                  end
                  i32.const 92
                  local.set 0
                  i32.const 1
                  local.set 1
                end
                local.get 5
                local.get 0
                local.get 7
                call_indirect (type 1)
                i32.eqz
                br_if 0 (;@6;)
              end
            end
          end
          i32.const 1
          local.set 0
          br 2 (;@1;)
        end
        local.get 4
        local.get 3
        local.get 8
        local.get 10
        call $_ZN4core3str16slice_error_fail17h2d866f60c2c4dd12E
        unreachable
      end
      unreachable
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0
    local.get 0)
  (func $memcpy (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    i32.const 0
    local.set 3
    block  ;; label = @1
      loop  ;; label = @2
        local.get 2
        local.get 3
        i32.eq
        br_if 1 (;@1;)
        local.get 0
        local.get 3
        i32.add
        local.get 1
        local.get 3
        i32.add
        i32.load8_u
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.set 3
        br 0 (;@2;)
      end
    end
    local.get 0)
  (func $memmove (type 0) (param i32 i32 i32) (result i32)
    (local i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        local.get 1
        i32.sub
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        loop  ;; label = @3
          local.get 2
          i32.eqz
          br_if 2 (;@1;)
          local.get 3
          local.get 1
          i32.load8_u
          i32.store8
          local.get 2
          i32.const -1
          i32.add
          local.set 2
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          br 0 (;@3;)
        end
      end
      local.get 1
      i32.const -1
      i32.add
      local.set 1
      local.get 0
      i32.const -1
      i32.add
      local.set 3
      loop  ;; label = @2
        local.get 2
        i32.eqz
        br_if 1 (;@1;)
        local.get 3
        local.get 2
        i32.add
        local.get 1
        local.get 2
        i32.add
        i32.load8_u
        i32.store8
        local.get 2
        i32.const -1
        i32.add
        local.set 2
        br 0 (;@2;)
      end
    end
    local.get 0)
  (func $memcmp (type 0) (param i32 i32 i32) (result i32)
    (local i32 i32)
    loop  ;; label = @1
      block  ;; label = @2
        local.get 2
        br_if 0 (;@2;)
        i32.const 0
        return
      end
      local.get 2
      i32.const -1
      i32.add
      local.set 2
      local.get 1
      i32.load8_u
      local.set 3
      local.get 0
      i32.load8_u
      local.set 4
      local.get 0
      i32.const 1
      i32.add
      local.set 0
      local.get 1
      i32.const 1
      i32.add
      local.set 1
      local.get 4
      local.get 3
      i32.eq
      br_if 0 (;@1;)
    end
    local.get 4
    local.get 3
    i32.sub)
  (table (;0;) 24 24 funcref)
  (global (;0;) (mut i32) (i32.const 65536))
  (global (;1;) i32 (i32.const 84912))
  (global (;2;) i32 (i32.const 84912))
  (export "deploy" (func $deploy))
  (export "call" (func $call))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func $_ZN70_$LT$core..result..Result$LT$T$C$E$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hb6cda997336d4a2dE $_ZN58_$LT$ink_env..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h47fa7ba2ded42968E $_ZN4core3ops8function6FnOnce9call_once17h01f7e944991b7a5dE $_ZN4core3ptr58drop_in_place$LT$$RF$psp22..traits..PSP22ReceiverError$GT$17he5a41f53852e4afeE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h3d900fd5aef00106E $_ZN4core3ptr33drop_in_place$LT$$RF$$LP$$RP$$GT$17h79d37317ba0fbc4fE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17haf075c85251a1022E $_ZN4core3ptr54drop_in_place$LT$$RF$mut$u20$alloc..string..String$GT$17h6d097882788d66fbE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17hfb52867266a9c081E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h486b065076821b1bE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h8ae99549e2ec2661E $_ZN4core3ptr52drop_in_place$LT$core..fmt..builders..PadAdapter$GT$17h0124eecc74b0e386E $_ZN68_$LT$core..fmt..builders..PadAdapter$u20$as$u20$core..fmt..Write$GT$9write_str17h2d434b5b7cb25cb0E $_ZN4core3fmt5Write10write_char17h20fafbc9c0c3193aE $_ZN4core3fmt5Write9write_fmt17h3247292a22916bc3E $_ZN4core3ptr64drop_in_place$LT$$RF$mut$u20$core..fmt..builders..PadAdapter$GT$17h2e2e01f8b4da3bd1E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_str17h7dd5d6dba37b9558E $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$10write_char17h1077bc9367c82bfeE $_ZN50_$LT$$RF$mut$u20$W$u20$as$u20$core..fmt..Write$GT$9write_fmt17h5366af33edc29456E $_ZN4core3ptr57drop_in_place$LT$$RF$parity_scale_codec..error..Error$GT$17h5841659b91333c2fE $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h14efe555c0bd4671E $_ZN4core3ptr46drop_in_place$LT$$RF$alloc..string..String$GT$17hbb500e04f37bfd59E $_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17hf89e08cb0223db08E)
  (data (;0;) (i32.const 65536) "Unknown error: call failed with \00\00\01\00 \00\00\00The contract with `to` address does not accept tokens: \00(\00\01\007\00\00\00Err\00\04\00\00\00\04\00\00\00\04\00\00\00\05\00\00\00Ok\00\00\06\00\00\00\04\00\00\00\04\00\00\00\07\00\00\00I hate this account!\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\04\00\00\00\04\00\00\00\09\00\00\00\0a\00\00\00\0b\00\00\00\0c\00\00\00\0c\00\00\00\04\00\00\00\0d\00\00\00\0e\00\00\00\0f\00\00\00    ,\0a, (\0a(,)\00\00\00\10\00\00\00\04\00\00\00\04\00\00\00\11\00\00\00\12\00\00\00\13\00\00\00()\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00\00\01\03\05\05\06\06\03\07\06\08\08\09\11\0a\1c\0b\19\0c\14\0d\10\0e\0d\0f\04\10\03\12\12\13\09\16\01\17\05\18\02\19\03\1a\07\1c\02\1d\01\1f\16 \03+\03,\02-\0b.\010\031\022\01\a7\02\a9\02\aa\04\ab\08\fa\02\fb\05\fd\04\fe\03\ff\09\adxy\8b\8d\a20WX\8b\8c\90\1c\1d\dd\0e\0fKL\fb\fc./?\5c]_\b5\e2\84\8d\8e\91\92\a9\b1\ba\bb\c5\c6\c9\ca\de\e4\e5\ff\00\04\11\12)147:;=IJ]\84\8e\92\a9\b1\b4\ba\bb\c6\ca\ce\cf\e4\e5\00\04\0d\0e\11\12)14:;EFIJ^de\84\91\9b\9d\c9\ce\cf\0d\11)EIWde\8d\91\a9\b4\ba\bb\c5\c9\df\e4\e5\f0\0d\11EIde\80\84\b2\bc\be\bf\d5\d7\f0\f1\83\85\8b\a4\a6\be\bf\c5\c7\ce\cf\da\dbH\98\bd\cd\c6\ce\cfINOWY^_\89\8e\8f\b1\b6\b7\bf\c1\c6\c7\d7\11\16\17[\5c\f6\f7\fe\ff\80\0dmq\de\df\0e\0f\1fno\1c\1d_}~\ae\af\bb\bc\fa\16\17\1e\1fFGNOXZ\5c^~\7f\b5\c5\d4\d5\dc\f0\f1\f5rs\8ftu\96/_&./\a7\af\b7\bf\c7\cf\d7\df\9a@\97\980\8f\1f\c0\c1\ce\ffNOZ[\07\08\0f\10'/\ee\efno7=?BE\90\91\fe\ffSgu\c8\c9\d0\d1\d8\d9\e7\fe\ff\00 _\22\82\df\04\82D\08\1b\04\06\11\81\ac\0e\80\ab5(\0b\80\e0\03\19\08\01\04/\044\04\07\03\01\07\06\07\11\0aP\0f\12\07U\07\03\04\1c\0a\09\03\08\03\07\03\02\03\03\03\0c\04\05\03\0b\06\01\0e\15\05:\03\11\07\06\05\10\07W\07\02\07\15\0dP\04C\03-\03\01\04\11\06\0f\0c:\04\1d%_ m\04j%\80\c8\05\82\b0\03\1a\06\82\fd\03Y\07\15\0b\17\09\14\0c\14\0cj\06\0a\06\1a\06Y\07+\05F\0a,\04\0c\04\01\031\0b,\04\1a\06\0b\03\80\ac\06\0a\06!?L\04-\03t\08<\03\0f\03<\078\08+\05\82\ff\11\18\08/\11-\03 \10!\0f\80\8c\04\82\97\19\0b\15\88\94\05/\05;\07\02\0e\18\09\80\b3-t\0c\80\d6\1a\0c\05\80\ff\05\80\df\0c\ee\0d\03\84\8d\037\09\81\5c\14\80\b8\08\80\cb*8\03\0a\068\08F\08\0c\06t\0b\1e\03Z\04Y\09\80\83\18\1c\0a\16\09L\04\80\8a\06\ab\a4\0c\17\041\a1\04\81\da&\07\0c\05\05\80\a5\11\81m\10x(*\06L\04\80\8d\04\80\be\03\1b\03\0f\0d\00\06\01\01\03\01\04\02\08\08\09\02\0a\05\0b\02\0e\04\10\01\11\02\12\05\13\11\14\01\15\02\17\02\19\0d\1c\05\1d\08$\01j\03k\02\bc\02\d1\02\d4\0c\d5\09\d6\02\d7\02\da\01\e0\05\e1\02\e8\02\ee \f0\04\f8\02\f9\02\fa\02\fb\01\0c';>NO\8f\9e\9e\9f\06\07\096=>V\f3\d0\d1\04\14\1867VW\7f\aa\ae\af\bd5\e0\12\87\89\8e\9e\04\0d\0e\11\12)14:EFIJNOde\5c\b6\b7\1b\1c\07\08\0a\0b\14\1769:\a8\a9\d8\d9\097\90\91\a8\07\0a;>fi\8f\92o_\ee\efZb\9a\9b'(U\9d\a0\a1\a3\a4\a7\a8\ad\ba\bc\c4\06\0b\0c\15\1d:?EQ\a6\a7\cc\cd\a0\07\19\1a\22%>?\c5\c6\04 #%&(38:HJLPSUVXZ\5c^`cefksx}\7f\8a\a4\aa\af\b0\c0\d0\ae\afy\ccno\93^\22{\05\03\04-\03f\03\01/.\80\82\1d\031\0f\1c\04$\09\1e\05+\05D\04\0e*\80\aa\06$\04$\04(\084\0b\01\80\90\817\09\16\0a\08\80\989\03c\08\090\16\05!\03\1b\05\01@8\04K\05/\04\0a\07\09\07@ '\04\0c\096\03:\05\1a\07\04\0c\07PI73\0d3\07.\08\0a\81&RN(\08*V\1c\14\17\09N\04\1e\0fC\0e\19\07\0a\06H\08'\09u\0b?A*\06;\05\0a\06Q\06\01\05\10\03\05\80\8bb\1eH\08\0a\80\a6^\22E\0b\0a\06\0d\139\07\0a6,\04\10\80\c0<dS\0cH\09\0aFE\1bH\08S\1d9\81\07F\0a\1d\03GI7\03\0e\08\0a\069\07\0a\816\19\80\b7\01\0f2\0d\83\9bfu\0b\80\c4\8a\bc\84/\8f\d1\82G\a1\b9\829\07*\04\02`&\0aF\0a(\05\13\82\b0[eK\049\07\11@\05\0b\02\0e\97\f8\08\84\d6*\09\a2\f7\81\1f1\03\11\04\08\81\8c\89\04k\05\0d\03\09\07\10\93`\80\f6\0as\08n\17F\80\9a\14\0cW\09\19\80\87\81G\03\85B\0f\15\85P+\80\d5-\03\1a\04\02\81p:\05\01\85\00\80\d7)L\04\0a\04\02\83\11DL=\80\c2<\06\01\04U\05\1b4\02\81\0e,\04d\0cV\0a\80\ae8\1d\0d,\04\09\07\02\0e\06\80\9a\83\d8\08\0d\03\0d\03t\0cY\07\0c\14\0c\048\08\0a\06(\08\22N\81T\0c\15\03\03\05\07\09\19\07\07\09\03\0d\07)\80\cb%\0a\84\06\00\00\00\00\03\00\00\83\04 \00\91\05`\00]\13\a0\00\12\17\a0\1e\0c \e0\1e\ef, +*0\a0+o\a6`,\02\a8\e0,\1e\fb\e0-\00\fe\a05\9e\ff\e05\fd\01a6\01\0a\a16$\0da7\ab\0e\e18/\18!90\1caF\f3\1e\a1J\f0jaNOo\a1N\9d\bc!Oe\d1\e1O\00\da!P\00\e0\e1Q0\e1aS\ec\e2\a1T\d0\e8\e1T \00.U\f0\01\bfU\00p\00\07\00-\01\01\01\02\01\02\01\01H\0b0\15\10\01e\07\02\06\02\02\01\04#\01\1e\1b[\0b:\09\09\01\18\04\01\09\01\03\01\05+\03w\0f\01 7\01\01\01\04\08\04\01\03\07\0a\02\1d\01:\01\01\01\02\04\08\01\09\01\0a\02\1a\01\02\029\01\04\02\04\02\02\03\03\01\1e\02\03\01\0b\029\01\04\05\01\02\04\01\14\02\16\06\01\01:\01\01\02\01\04\08\01\07\03\0a\02\1e\01;\01\01\01\0c\01\09\01(\01\03\019\03\05\03\01\04\07\02\0b\02\1d\01:\01\02\01\02\01\03\01\05\02\07\02\0b\02\1c\029\02\01\01\02\04\08\01\09\01\0a\02\1d\01H\01\04\01\02\03\01\01\08\01Q\01\02\07\0c\08b\01\02\09\0b\06J\02\1b\01\01\01\01\017\0e\01\05\01\02\05\0b\01$\09\01f\04\01\06\01\02\02\02\19\02\04\03\10\04\0d\01\02\02\06\01\0f\01\00\03\00\03\1d\03\1d\02\1e\02@\02\01\07\08\01\02\0b\09\01-\03w\02\22\01v\03\04\02\09\01\06\03\db\02\02\01:\01\01\07\01\01\01\01\02\08\06\0a\02\010\11?\040\07\01\01\05\01(\09\0c\02 \04\02\02\01\038\01\01\02\03\01\01\03:\08\02\02\98\03\01\0d\01\07\04\01\06\01\03\02\c6:\01\05\00\01\c3!\00\03\8d\01` \00\06i\02\00\04\01\0a \02P\02\00\01\03\01\04\01\19\02\05\01\97\02\1a\12\0d\01&\08\19\0b.\030\01\02\04\02\02'\01C\06\02\02\02\02\0c\01\08\01/\013\01\01\03\02\02\05\02\01\01*\02\08\01\ee\01\02\01\04\01\00\01\00\10\10\10\00\02\00\01\e2\01\95\05\00\03\01\02\05\04(\03\04\01\a5\02\00\04\00\02\99\0b\b0\016\0f8\031\04\02\02E\03$\05\01\08>\01\0c\024\09\0a\04\02\01_\03\02\01\01\02\06\01\a0\01\03\08\15\029\02\01\01\01\01\16\01\0e\07\03\05\c3\08\02\03\01\01\17\01Q\01\02\06\01\01\02\01\01\02\01\02\eb\01\02\04\06\02\01\02\1b\02U\08\02\01\01\02j\01\01\01\02\06\01\01e\03\02\04\01\05\00\09\01\02\f5\01\0a\02\01\01\04\01\90\04\02\02\04\01 \0a(\06\02\04\08\01\09\06\02\03.\0d\01\02\00\07\01\06\01\01R\16\02\07\01\02\01\02z\06\03\01\01\02\01\07\01\01H\02\03\01\01\01\00\02\00\05;\07\00\01?\04Q\01\00\02\00\01\01\03\04\05\08\08\02\07\1e\04\94\03\007\042\08\01\0e\01\16\05\01\0f\00\07\01\11\02\07\01\02\01\05\00\07\00\04\00\07m\07\00`\80\f0\00EcdsaRecoverFailedLoggingDisabledUnknownNotCallableCodeNotFoundNewContractNotFundedTransferFailedBelowSubsistenceThresholdKeyNotFoundCalleeRevertedCalleeTrappedDecode\00\14\00\00\00\04\00\00\00\04\00\00\00\15\00\00\00Error\00\00\00\16\00\00\00\04\00\00\00\04\00\00\00\17\00\00\00TransferRejected\0d\00\00\00\01\00\00\00\02\00\00\00\03\00\00\00\04\00\00\00\05\00\00\00\06\00\00\00\07\00\00\00\08\00\00\00\09\00\00\00\0c\00\00\00\0b\00\00\00")
  (data (;1;) (i32.const 68520) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))
