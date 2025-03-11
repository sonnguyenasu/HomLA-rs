#![allow(unused)]
#![allow(non_snake_case)]
use tfhe::array::Slicing;
use tfhe::{prelude::*, CpuFheUint8Array, FheBool, FheUint, FheUint8Id};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8, FheInt8, FheInt8Id};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::default().build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);

    // On the server side:
    set_server_key(server_keys);

    // problem parameter
    const n:i8 = 7;
    const m:i8 = 9;
    const u:i8 = 1;
    const v:i8 = 9;
    const sO:i8 = 5;
    const sE:i8 = 3;

    let query = [4i8,3,0,3,1,0,3,0];
    let reference = [4i8,0,2,0,3,2,1,0,2,2];
    
    let query_enc = query.iter().map(|&val| FheInt8::try_encrypt(val, &client_key).unwrap()).collect::<Vec<_>>();
    let reference_enc = reference.iter().map(|&val| FheInt8::try_encrypt(val, &client_key).unwrap()).collect::<Vec<_>>();

    //let k: u8 = sE - sO;
    //println!("{}",k);
    
    let sO_enc = FheInt8::try_encrypt(sO, &client_key)?;
    let sE_enc = FheInt8::try_encrypt(sE, &client_key)?;
    let u_enc = FheInt8::try_encrypt(u, &client_key)?;
    let v_enc = FheInt8::try_encrypt(v, &client_key)?;
    let k = &sE_enc - &sO_enc;
    let k_lt0_enc = k.lt(0i8);
    let k_dec: bool = k_lt0_enc.decrypt(&client_key);

    println!("{}",k_dec);

    let init_h = [0i8;80].into_iter().collect::<Vec<_>>();
    let init_p = init_h.clone();
    let init_q = init_h.clone();
    let mut init_hx = init_h.clone();
    for i in 0..8{
        init_hx[i*10] = i as i8;
    }
    let init_px = init_h.clone();
    let mut init_hy = init_h.clone();
    for j in 0..10{
        init_hy[j] = j as i8;
    }
    let init_py = init_hy.clone();
    let init_qy = init_h.clone();


    // initialize the encrypted tables
    let mut H : Vec<_>= init_h.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut H:Vec<_> = H.chunks_mut(10).collect();
    let mut Hx : Vec<_>= init_hx.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut Hx:Vec<_> = Hx.chunks_mut(10).collect();
    let mut Hy: Vec<_> = init_hy.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut Hy:Vec<_> = Hy.chunks_mut(10).collect();
    let mut P : Vec<_>= init_p.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut P:Vec<_> = P.chunks_mut(10).collect();
    let mut Px : Vec<_>= init_px.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut Px:Vec<_> = Px.chunks_mut(10).collect();
    let mut Py : Vec<_>= init_py.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut Py:Vec<_> = Py.chunks_mut(10).collect();
    let mut Q : Vec<_>= init_q.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut Q:Vec<_> = Q.chunks_mut(10).collect();
    let mut Qx : Vec<_>= init_hx.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut Qx:Vec<_> = Qx.chunks_mut(10).collect();
    let mut Qy : Vec<_>= init_qy.iter().enumerate().map(|(i, &score)|{
        FheInt8::try_encrypt(score, &client_key).unwrap()
    }).collect();
    let mut Qy:Vec<_> = Qy.chunks_mut(10).collect();
    //let mut H = CpuFheUint8Array::try_encrypt((init_h.as_slice(), vec![8,10]), &client_key)?;
    //let mut P= CpuFheUint8Array::try_encrypt((init_p.as_slice(), vec![8,10]), &client_key)?;

    //let mut Q= CpuFheUint8Array::try_encrypt((init_q.as_slice(), vec![8,10]), &client_key)?;

    //let mut Hx= CpuFheUint8Array::try_encrypt((init_hx.as_slice(), vec![8,10]), &client_key)?;

    //let mut Px= CpuFheUint8Array::try_encrypt((init_px.as_slice(), vec![8,10]), &client_key)?;

    //let mut Qx= CpuFheUint8Array::try_encrypt((init_hx.as_slice(), vec![8,10]), &client_key)?;

    //let mut Hy= CpuFheUint8Array::try_encrypt((init_hy.as_slice(), vec![8,10]), &client_key)?;

    //let mut Py= CpuFheUint8Array::try_encrypt((init_py.as_slice(), vec![8,10]), &client_key)?;

    //let mut Qy= CpuFheUint8Array::try_encrypt((init_qy.as_slice(), vec![8,10]), &client_key)?;
    let mut Es: Vec<Vec<FheBool>> = vec![];
    for i in 0..8{
        Es.push(vec![]);
        for j in 0..10{
            Es[i].push(query_enc[i].eq(reference[j]));
        }
    }
    
    let d_decrypted: Vec<i8> = Hy[0][..10].iter().map(|enc| enc.decrypt(&client_key)).collect();
    for i in 0..10{
        print!("{} ",d_decrypted[i]);
    }
    println!("");
    let now = std::time::Instant::now();
    // begin computing the matrix
    for d in 1..n+m-1{
        let start = if d-m > 0 { d-m+1 } else { 1 };
        let end = if d < n { d } else {n};
        for k in start..=end{
            let i:usize = k as usize;
            let j:usize = (d - k + 1) as usize;
            if i >= n as usize || j >= m as usize { 
                continue;
            }
            //println!("{} {}", i, j);
            let T1 = &P[i-1][j] - u;
            let T2 = &H[i-1][j] - v;
            let S = T1.ge(&T2);
            P[i][j] = S.select(&T1, &T2);
            Px[i][j] = S.select(&Px[i-1][j], &Hx[i-1][j]);
            Py[i][j] = S.select(&Py[i-1][j],&Hy[i-1][j]);
            let T1 = &Q[i][j-1] - u;
            let T2 = &H[i][j-1] - v;
            let S = T1.ge(&T2);
            Q[i][j] = S.select(&T1, &T2);
            Qx[i][j] = S.select(&Qx[i][j-1], &Hx[i][j-1]);
            Qy[i][j] = S.select(&Qy[i][j-1],&Hy[i][j-1]);
            let S = P[i][j].ge(&Q[i][j]);
            H[i][j] = S.select(&P[i][j], &Q[i][j]);
            Hx[i][j] = S.select(&Px[i][j], &Qx[i][j]);
            Hy[i][j] = S.select(&Py[i][j], &Qy[i][j]);
            let E = Es[i][j].clone();
            let T2 = &H[i-1][j-1] - sE;
            let T1 = &H[i-1][j-1] + sO;
            let T2 = E.select(&T1, &T2);
            let S = T2.ge(&H[i][j]);
            H[i][j] = S.select(&T2, &H[i][j]);
            Hx[i][j] = S.select(&Hx[i-1][j-1], &Hx[i][j]);
            Hy[i][j] = S.select(&Hy[i-1][j-1], &Hy[i][j]);
            let S = H[i][j].ge(0);
            let S_dec: bool = S.decrypt(&client_key);
            //println!("{}", S_dec);
            H[i][j] = S.select(&H[i][j], &FheInt8::encrypt(0i8, &client_key));
            Hx[i][j] = S.select(&Hx[i][j], &FheInt8::encrypt(i as i8, &client_key));
            Hy[i][j] = S.select(&Hy[i][j], &FheInt8::encrypt(j as i8, &client_key));
        }
    }
    let elapsed = now.elapsed();
    println!("compute time: {} s", elapsed.as_secs());
    for i in 0..8{
        for j in 0..10{
            let d_decrypted: i8 = H[i][j].decrypt(&client_key);
            print!("{} ", d_decrypted);
        }
        println!("");
    }
    Ok(())
}
